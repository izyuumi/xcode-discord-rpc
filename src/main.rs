use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc,
};
use simple_logger::SimpleLogger;

mod config;
mod error;
mod utils;

use config::AppConfig;
#[allow(unused)]
pub use error::{Error, Result};
use utils::{file_language::*, osascript::*, *};

fn main() -> Result<()> {
    SimpleLogger::new().init()?;

    let config = AppConfig::new()?;

    loop {
        if let Err(err) = discord_rpc(&config) {
            log::warn!("Failed to connect to Discord: {err}");
            log::debug!("Trying to reconnect...");
            sleep(config.update_interval)
        }
        sleep(config.update_interval)
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn discord_rpc(config: &AppConfig) -> Result<()> {
    let mut client = init_discord_ipc()?;

    let mut xcode_is_running = false;
    let mut xcode_check_cycle_counter = 0;

    loop {
        if xcode_check_cycle_counter == config.xcode_check_cycle {
            xcode_check_cycle_counter = 0;
            xcode_is_running = check_xcode()?;
            if !xcode_is_running {
                log::info!("Xcode is not running");
                sleep(config.update_interval);
                continue;
            }
        }
        xcode_check_cycle_counter += 1;

        if client.connect().is_ok() {
            log::info!("Connected to Discord");
            let mut started_at = Timestamps::new().start(current_time() * 1000);
            let mut project_before = String::from("");
            let mut last_frontmost_at = current_time();

            while xcode_is_running {
                log::debug!("Xcode is running");
                let project = if config.hide_project {
                    String::from("")
                } else {
                    current_project()?
                };

                if is_xcode_frontmost()? {
                    last_frontmost_at = current_time();
                }
                let is_idle = current_time() - last_frontmost_at > config.idle_threshold;

                if !project_before.eq(&project) {
                    started_at = Timestamps::new().start(current_time() * 1000);
                    project_before = project.clone();
                }

                if project.is_empty() || is_idle {
                    client.set_activity(
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
                    sleep(config.update_interval);
                    xcode_is_running = check_xcode()?;
                    continue;
                }

                let mut keys = FileLanguage::Unknown.get_asset_keys();

                let details = if config.hide_file {
                    "Working on a file"
                } else {
                    let file = current_file()?;
                    let file_extension = file.get_file_extension();
                    keys = file_extension.to_file_language().get_asset_keys();
                    &format!("Working on {}", file)
                };

                let state = if config.hide_project {
                    "in a Project"
                } else {
                    &format!("in {}", project)
                };

                let activity = Activity::new()
                    .timestamps(started_at.clone())
                    .assets(Assets::new().large_text(keys.0).large_image(keys.1))
                    .details(details)
                    .state(state);

                client.set_activity(activity)?;
                log::debug!("Updated activity: working on a project");

                sleep(config.update_interval);
                xcode_is_running = check_xcode()?
            }
        } else {
            log::debug!("Discord is not running");
        }
        sleep(config.update_interval);
    }

    #[allow(unreachable_code)]
    Ok(())
}
