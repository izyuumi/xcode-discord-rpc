use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::{
    config::AppConfig,
    utils::{
        current_time,
        file_language::{FileExtention, FileLanguage, ToFileLanguage},
        osascript::{check_xcode, current_file, current_project, is_xcode_frontmost},
        sleep,
    },
    Result,
};

enum Flow {
    /// `continue` to the next loop
    Continue(()),
    /// Run the next line
    GoNext,
}

pub struct XcodeState<'a> {
    xcode_is_running: bool,
    xcode_check_cycle_counter: u8,
    config: &'a AppConfig,
    discord_ipc: &'a mut DiscordIpcClient,
    discord_is_connected: bool,
    /// Multiplier used to progressively increase sleep duration when Xcode or
    /// Discord is not running. This helps reduce CPU usage when idle.
    sleep_multiplier: u64,
}

impl<'a> XcodeState<'a> {
    /// Creates a new XcodeState instance with the provided configuration and Discord IPC client
    pub fn new(config: &'a AppConfig, discord_ipc: &'a mut DiscordIpcClient) -> Self {
        Self {
            xcode_is_running: false,
            xcode_check_cycle_counter: config.xcode_check_cycle,
            config,
            discord_ipc,
            discord_is_connected: false,
            sleep_multiplier: 1,
        }
    }

    /// Runs the main loop that monitors Xcode and updates Discord Rich Presence
    pub fn run(&mut self) -> Result<()> {
        loop {
            // check xcode
            if let Flow::Continue(()) = self.check_xcode_cycle()? {
                continue;
            };

            // make sure discord is running and we are connected
            if let Err(e) = self.discord_ipc.connect() {
                log::debug!("Discord is not running: {}", e);
                self.discord_is_connected = false;
                self.increase_sleep_multiplier();
                self.sleep_discord_xcode();
                continue;
            }
            self.discord_is_connected = true;
            self.reset_sleep_multiplier();

            log::info!("Connected to Discord");
            self.handle_discord_session()?;

            self.sleep_xcode_update();
        }

        #[allow(unreachable_code)]
        Ok(())
    }

    /// Sleep for the configured update interval to check if Xcode/Discord is running
    fn sleep_discord_xcode(&self) {
        sleep(self.config.update_interval * self.sleep_multiplier);
    }

    /// Sleep for the configured Xcode update interval to check for updates
    fn sleep_xcode_update(&self) {
        sleep(self.config.xcode_update_interval);
    }

    /// Increase the sleep multiplier using exponential backoff with a maximum
    /// cap to avoid excessively frequent checks when Xcode or Discord are not
    /// running.
    fn increase_sleep_multiplier(&mut self) {
        const MAX_MULTIPLIER: u64 = 8;
        self.sleep_multiplier = (self.sleep_multiplier * 2).min(MAX_MULTIPLIER);
    }

    /// Reset the sleep multiplier when Xcode and Discord are running again.
    fn reset_sleep_multiplier(&mut self) {
        self.sleep_multiplier = 1;
    }
}

/// Xcode-related internal functions for `XcodeState`
impl XcodeState<'_> {
    /// Checks if Xcode is running and updates internal state
    fn check_xcode(&mut self) -> Result<()> {
        self.xcode_is_running = check_xcode()?;
        self.xcode_check_cycle_counter = 0;
        Ok(())
    }

    /// Handles periodic Xcode check logic and determines flow control
    fn check_xcode_cycle(&mut self) -> Result<Flow> {
        log::debug!("Checking Xcode cycle: {}", self.xcode_check_cycle_counter);
        if self.xcode_check_cycle_counter == self.config.xcode_check_cycle {
            self.xcode_check_cycle_counter = 0;
            self.check_xcode()?;
            if !self.xcode_is_running {
                log::debug!("Xcode is not running");
                if self.discord_is_connected {
                    self.clear_activity()?;
                }
                self.increase_sleep_multiplier();
                self.sleep_discord_xcode();
                return Ok(Flow::Continue(()));
            }
            self.reset_sleep_multiplier();
        }
        self.xcode_check_cycle_counter += 1;

        if !self.xcode_is_running {
            self.increase_sleep_multiplier();
            self.sleep_discord_xcode();
            return Ok(Flow::Continue(()));
        }

        self.reset_sleep_multiplier();

        Ok(Flow::GoNext)
    }
}

/// Discord-related internal functions for `XcodeState`
impl XcodeState<'_> {
    /// Manages the Discord session and continuously updates Rich Presence based on Xcode activity
    fn handle_discord_session(&mut self) -> Result<()> {
        let mut started_at = Timestamps::new().start(current_time() * 1000);
        let mut project_before = String::from("");
        let mut last_frontmost_at = current_time();

        self.reset_sleep_multiplier();

        while self.xcode_is_running {
            log::debug!("Xcode is running");

            self.update_frontmost_time(&mut last_frontmost_at)?;
            let project = self.get_current_project()?;

            if !project_before.eq(&project) {
                started_at = Timestamps::new().start(current_time() * 1000);
                project_before = project.clone();
            }

            let is_idle = current_time() - last_frontmost_at > self.config.idle_threshold;

            if project.is_empty() || is_idle {
                self.set_idle_activity(&started_at)?;
                continue;
            }

            self.set_working_activity(&project, &started_at)?;
            self.sleep_xcode_update();
            self.check_xcode()?;
        }
        Ok(())
    }

    /// Updates the timestamp for when Xcode was last in the foreground
    fn update_frontmost_time(&self, last_frontmost_at: &mut i64) -> Result<()> {
        if is_xcode_frontmost()? {
            *last_frontmost_at = current_time();
        }
        Ok(())
    }

    /// Retrieves current project name, respecting hide_project configuration
    fn get_current_project(&self) -> Result<String> {
        if self.config.hide_project {
            Ok(String::from(""))
        } else {
            current_project()
        }
    }

    /// Sets Discord activity to idle state
    fn set_idle_activity(&mut self, started_at: &Timestamps) -> Result<()> {
        self.discord_ipc.set_activity(
            Activity::new()
                .timestamps(started_at.clone())
                .assets(
                    Assets::new()
                        .large_text(FileLanguage::Unknown.get_text_asset_key())
                        .large_image(FileLanguage::Unknown.get_image_asset_key()),
                )
                .details("Idle")
                .state("Idle"),
        )?;
        log::info!("Updated activity: idle");
        self.increase_sleep_multiplier();
        self.sleep_discord_xcode();
        self.check_xcode()?;
        Ok(())
    }

    /// Sets Discord activity to working state with project and file information
    fn set_working_activity(&mut self, project: &str, started_at: &Timestamps) -> Result<()> {
        // Get all data first
        let (details, (large_text, large_image)) = self.get_file_details()?;
        let state = self.get_project_state(project);

        // Now use the data to set activity
        let activity = Activity::new()
            .timestamps(started_at.clone())
            .assets(
                Assets::new()
                    .large_text(&large_text)
                    .large_image(&large_image),
            )
            .details(&details)
            .state(&state);

        self.discord_ipc.set_activity(activity)?;
        log::debug!("Updated activity: working on a project");
        self.reset_sleep_multiplier();
        Ok(())
    }

    /// Clear the Discord activity
    fn clear_activity(&mut self) -> Result<()> {
        self.discord_ipc.clear_activity()?;
        Ok(())
    }

    /// Retrieves detailed information about current file for Discord Rich Presence
    fn get_file_details(&self) -> Result<(String, (String, String))> {
        let mut file_language = FileLanguage::Unknown;
        let mut keys = (
            String::from(file_language.get_text_asset_key()),
            String::from(file_language.get_image_asset_key()),
        );

        let details = if self.config.hide_file {
            String::from("Working on a file")
        } else {
            let file = current_file()?;
            let file_extension = file.get_file_extension();
            file_language = file_extension.to_file_language();
            keys = (
                String::from(file_language.get_text_asset_key()),
                String::from(file_language.get_image_asset_key()),
            );
            format!("Working on {file}")
        };

        Ok((details, keys))
    }

    /// Generates state text based on project name and configuration
    fn get_project_state(&self, project: &str) -> String {
        if self.config.hide_project {
            String::from("in a Project")
        } else {
            format!("in {project}")
        }
    }
}
