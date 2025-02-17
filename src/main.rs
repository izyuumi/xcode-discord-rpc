use chrono::Local;
use config::AppConfig;
use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    process::Command,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod config;
mod error;

#[allow(unused)]
pub(crate) use error::{Error, Result};

fn main() {
    let Ok(config) = AppConfig::new() else {
        log("Failed to load configuration", None);
        log("Exiting...", None);
        std::process::exit(1);
    };

    loop {
        if let Err(err) = discord_rpc(&config) {
            log("Failed to connect to Discord", Some(&err.to_string()));
            log("Trying to reconnect...", None);
            sleep(config.wait_time)
        }
        sleep(config.wait_time)
    }
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
                log("Xcode is not running", None);
                sleep(config.wait_time);
                continue;
            }
        }
        xcode_check_cycle_counter += 1;

        if client.connect().is_ok() {
            log("Connected to Discord", None);
            let mut started_at = Timestamps::new().start(current_time() * 1000);
            let mut project_before = String::from("");
            let mut last_frontmost_at = current_time();

            while xcode_is_running {
                log("Xcode is running", None);
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
                                    .large_image("xcode")
                                    .large_text("Xcode")
                                    .small_image("xcode")
                                    .small_text("Xcode"),
                            )
                            .details("Idle")
                            .state("Idle"),
                    )?;
                    log("Updated activity: idle", None);
                    sleep(config.wait_time);
                    xcode_is_running = check_xcode()?;
                    continue;
                }

                let mut keys = ("Xcode", "xcode");

                let details = if config.hide_file {
                    "Working on a file"
                } else {
                    let file = current_file()?;
                    let file_extension = (file.split('.').last().unwrap_or("")).trim().to_string();
                    keys = match file_extension.as_str() {
                        "swift" => ("Swift", "swift"),
                        "cpp" | "cp" | "cxx" => ("C++", "cpp"),
                        "c" => ("C", "c"),
                        "rb" => ("Ruby", "ruby"),
                        "java" => ("Java", "java"),
                        "json" => ("JSON", "json"),
                        "metal" => ("Metal", "metal"),
                        _ => ("Xcode", "xcode"),
                    };
                    &format!("Working on {}", file)
                };

                let state = if config.hide_project {
                    "in a Project"
                } else {
                    &format!("in {}", project)
                };

                let activity = Activity::new()
                    .timestamps(started_at.clone())
                    .assets(Assets::new().large_image(keys.1).large_text(keys.0))
                    .details(details)
                    .state(state);

                client.set_activity(activity)?;
                log("Updated activity: working on a project", None);

                sleep(config.wait_time);
                xcode_is_running = check_xcode()?
            }
        } else {
            log("Discord is not running", None);
        }
        sleep(config.wait_time);
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
    )?
    .trim()
    .to_string();
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
    )?
    .trim()
    .to_string();
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
        .expect("Failed to execute command");
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Get the current time in seconds since the UNIX epoch as a 64-bit integer
fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to obtain current time")
        .as_secs() as i64
}

/// Standardized logging function
fn log(message: &str, error: Option<&str>) {
    let date_format = Local::now().format("%Y-%m-%d %H:%M:%S");
    match error {
        Some(error) => eprintln!("{}: {} (Error: {})", date_format, message, error),
        None => println!("{}: {}", date_format, message),
    }
}

/// Sleep for WAIT_TIME seconds
fn sleep(wait_time: u64) {
    thread::sleep(Duration::from_secs(wait_time));
}

/// Check if frontmost application is Xcode
fn is_xcode_frontmost() -> Result<bool> {
    let frontmost_app = run_osascript(
        r#"
        if frontmost of application "Xcode" is true then
            return "Xcode"
        end if
    "#,
    )?
    .trim()
    .to_string();
    Ok(frontmost_app == "Xcode")
}
