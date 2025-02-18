use config::AppConfig;
use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use simple_logger::SimpleLogger;
use std::{
    process::Command,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod config;
mod error;
mod utils;

#[allow(unused)]
pub(crate) use error::{Error, Result};
use utils::file_language::{FileExtention, FileLanguage, ToFileLanguage};

fn main() -> Result<()> {
    SimpleLogger::new().init()?;

    let Ok(config) = AppConfig::new() else {
        log::warn!("Failed to load configuration");
        return Err(Error::Custom("Failed to load configuration".to_string()));
    };

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
    let mut client = match DiscordIpcClient::new("1158013054898950185") {
        Ok(client) => client,
        Err(err) => return Err(Error::DiscordIpc(err.to_string())),
    };

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

/// Check if Xcode is running
fn check_xcode() -> Result<bool> {
    let xcode_is_running = run_osascript(
        r#"
        tell application "System Events"
            set xcodeIsRunning to exists (processes where name is "Xcode")
        end tell
    "#,
    )?;
    Ok(xcode_is_running == "true")
}

/// Get the current file's name as a String
fn current_file() -> Result<String> {
    let file = run_osascript(
        r#"
        tell application "Xcode"
            return name of windows whose index is 1
        end tell
    "#,
    )?;
    if !file.contains(" — ") {
        return Ok(file);
    }
    let file = file.split(" — ").collect::<Vec<&str>>()[1];
    Ok(file.to_string())
}

/// Get the current project's name as a String
fn current_project() -> Result<String> {
    let project = run_osascript(
        r#"
        tell application "Xcode"
            return active workspace document
        end tell
    "#,
    )?;
    if project == "missing value" {
        return Ok(String::new());
    }
    if project.starts_with("workspace document ") {
        return Ok(project.replace("workspace document ", ""));
    }
    Ok(project)
}

/// Execute an AppleScript command using osascript and returns the output as a String
fn run_osascript(script: &str) -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|err| Error::Oascript(err.to_string()))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Get the current time in seconds since the UNIX epoch as `i64`
fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to obtain current time")
        .as_secs() as i64
}

/// Sleep for `Config::update_interval` seconds
fn sleep(update_interval: u64) {
    thread::sleep(Duration::from_secs(update_interval));
}

/// Check if frontmost application is Xcode
fn is_xcode_frontmost() -> Result<bool> {
    let frontmost_app = run_osascript(
        r#"
        if frontmost of application "Xcode" is true then
            return "Xcode"
        end if
    "#,
    )?;
    Ok(frontmost_app == "Xcode")
}
