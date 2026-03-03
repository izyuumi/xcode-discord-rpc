use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::{
    config::AppConfig,
    utils::{
        current_time,
        file_language::{FileExtention, FileLanguage, ToFileLanguage},
        git::{format_branch_state, get_git_branch, get_git_head_ref},
        osascript::{check_xcode, current_file, current_project, current_project_path, is_xcode_frontmost},
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
    /// TTL cache for the project path resolved via AppleScript.
    cached_project_path: Option<String>,
    /// TTL cache for the git branch resolved from the project path.
    cached_branch: Option<Option<String>>,
    /// Latest observed git HEAD marker for the cached project path.
    cached_head_ref: Option<Option<String>>,
    /// Timestamp of the last cache population.
    cache_populated_at: Option<Instant>,
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
            cached_project_path: None,
            cached_branch: None,
            cached_head_ref: None,
            cache_populated_at: None,
        }
    }

    /// Runs the main loop that monitors Xcode and updates Discord Rich Presence
    pub fn run(&mut self, running: &Arc<AtomicBool>) -> Result<()> {
        while running.load(Ordering::SeqCst) {
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
        let mut state_before: (String, Option<String>) = (String::from(""), None);
        let mut last_frontmost_at = current_time();

        self.reset_sleep_multiplier();

        while self.xcode_is_running {
            log::debug!("Xcode is running");

            self.update_frontmost_time(&mut last_frontmost_at)?;
            let project = self.get_current_project()?;

            let is_idle = !self.config.disable_idle
                && current_time() - last_frontmost_at > self.config.idle_threshold;

            if project.is_empty() || is_idle {
                self.set_idle_activity(&started_at)?;
                self.increase_sleep_multiplier();
                self.sleep_discord_xcode();
                self.check_xcode()?;
                continue;
            }

            // Resolve git branch from the active workspace document path.
            // Results are cached for BRANCH_CACHE_TTL, but invalidate early when
            // the repository HEAD changes so branch switches are reflected quickly.
            // Note: current_project_path() (AppleScript) still runs every update cycle.
            const BRANCH_CACHE_TTL: Duration = Duration::from_secs(30);
            let branch = if self.config.hide_branch {
                None
            } else {
                let project_path = match current_project_path() {
                    Ok(p) => p,
                    Err(e) => {
                        // Avoid leaking full filesystem paths in logs.
                        log::debug!("Failed to resolve active project path: {}", e);
                        String::new()
                    }
                };
                let project_changed = self
                    .cached_project_path
                    .as_deref()
                    .map(|p| p != project_path.as_str())
                    .unwrap_or(true);
                let head_ref = get_git_head_ref(&project_path);
                let head_changed = !project_changed
                    && self
                        .cached_head_ref
                        .as_ref()
                        .map(|cached| cached != &head_ref)
                        .unwrap_or(false);
                let cache_expired = project_changed
                    || head_changed
                    || self
                        .cache_populated_at
                        .map(|t| t.elapsed() >= BRANCH_CACHE_TTL)
                        .unwrap_or(true);

                if cache_expired {
                    // Log only the basename to avoid leaking full filesystem paths.
                    let path_basename = std::path::Path::new(&project_path)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    log::debug!("Resolving git branch for path: {:?}", path_basename);
                    let b = get_git_branch(&project_path);
                    // Log only whether a branch was found, not its name.
                    log::debug!("Git branch resolved: {}", if b.is_some() { "yes" } else { "no" });
                    self.cached_project_path = Some(project_path);
                    self.cached_branch = Some(b);
                    self.cached_head_ref = Some(head_ref);
                    self.cache_populated_at = Some(Instant::now());
                } else {
                    log::debug!("Using cached project path and git branch");
                }

                self.cached_branch.clone().flatten()
            };

            // Reset the session timer when either the project or git branch changes.
            // (This improves accuracy when switching branches without changing project.)
            let state_now = (project.clone(), branch.clone());
            if state_before != state_now {
                started_at = Timestamps::new().start(current_time() * 1000);
                state_before = state_now;
            }

            self.set_working_activity(&project, &started_at, branch.as_deref())?;
            self.sleep_xcode_update();
            self.check_xcode()?;
        }

        log::info!("Xcode stopped, clearing Discord activity");
        self.clear_activity()?;

        // Clear cached path/branch info so the next Xcode session starts fresh.
        self.cached_project_path = None;
        self.cached_branch = None;
        self.cached_head_ref = None;
        self.cache_populated_at = None;
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
        let project = current_project()?;
        if self.config.hide_project && !project.is_empty() {
            Ok(String::from("Project"))
        } else {
            Ok(project)
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
        Ok(())
    }

    /// Sets Discord activity to working state with project and file information
    fn set_working_activity(
        &mut self,
        project: &str,
        started_at: &Timestamps,
        branch: Option<&str>,
    ) -> Result<()> {
        // Get all data first
        let (details, (large_text, large_image)) = self.get_file_details()?;
        let state = self.get_project_state(project, branch);

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
    pub fn clear_activity(&mut self) -> Result<()> {
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

    /// Generates state text based on project name, configuration and optional git branch.
    ///
    /// When `hide_branch` is `false` and a branch name is available, the branch
    /// is appended to the state string with a bullet separator, e.g.
    /// `"in MyApp • feat/export"`. Long branch names are gracefully truncated
    /// so the combined string never exceeds Discord's 128-byte state limit.
    fn get_project_state(&self, project: &str, branch: Option<&str>) -> String {
        let base = if self.config.hide_project {
            String::from("in a Project")
        } else {
            format!("in {project}")
        };

        match branch {
            Some(b) => {
                format_branch_state(&base, b)
            }
            _ => base,
        }
    }
}
