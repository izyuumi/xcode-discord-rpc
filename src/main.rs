use chrono::Local;
use clap::{Arg, ArgAction, Command as ClapCommand};
use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    process::Command,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const WAIT_TIME: u64 = 30;
const XCODE_CHECK_CYCLE: i8 = 5;

const SHOW_FILE_ARG_ID: &str = "show_file";
const SHOW_PROJECT_ARG_ID: &str = "show_project";

fn main() {
    // Parse command-line arguments
    let matches = ClapCommand::new("Xcode Discord RPC")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Displays Xcode status on Discord Rich Presence")
        .arg(
            Arg::new(SHOW_FILE_ARG_ID)
                .short('f')
                .long("show-file")
                .num_args(0)
                .action(ArgAction::SetFalse)
                .help("Hide current file in Discord Rich Presence")
                .default_value("true"),
        )
        .arg(
            Arg::new(SHOW_PROJECT_ARG_ID)
                .short('p')
                .long("show-project")
                .num_args(0)
                .action(ArgAction::SetFalse)
                .help("Hide current project in Discord Rich Presence")
                .default_value("true"),
        )
        .get_matches();

    let (show_file, show_project) = (
        matches.get_flag(SHOW_FILE_ARG_ID),
        matches.get_flag(SHOW_PROJECT_ARG_ID),
    );

    loop {
        if let Err(err) = discord_rpc(show_file, show_project) {
            log("Failed to connect to Discord", Some(&err.to_string()));
            log("Trying to reconnect...", None);
            sleep()
        }
        sleep()
    }
}

fn discord_rpc(show_file: bool, show_project: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1158013054898950185")?;

    let mut xcode_is_running = false;
    let mut xcode_check_cycle_counter = 0;

    loop {
        if xcode_check_cycle_counter == XCODE_CHECK_CYCLE {
            xcode_check_cycle_counter = 0;
            xcode_is_running = check_xcode()?;
            if !xcode_is_running {
                log("Xcode is not running", None);
                sleep();
                continue;
            }
        }
        xcode_check_cycle_counter += 1;

        if client.connect().is_ok() {
            log("Connected to Discord", None);
            let mut started_at = Timestamps::new().start(current_time());
            let mut project_before = String::from("");

            while xcode_is_running {
                log("Xcode is running", None);
                let project = if show_project {
                    current_project()?
                } else {
                    String::from("")
                };

                if !project_before.eq(&project) {
                    started_at = Timestamps::new().start(current_time());
                    project_before = project.clone();
                }

                if project.is_empty() {
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
                    log("Updated activity", None);
                    sleep();
                    xcode_is_running = check_xcode()?;
                    continue;
                }

                let mut keys = ("Xcode", "xcode");

                let details = if show_file {
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
                } else {
                    "Working on a file"
                };

                let state = if show_project {
                    &format!("in {}", project)
                } else {
                    "in a Project"
                };

                let activity = Activity::new()
                    .timestamps(started_at.clone())
                    .assets(Assets::new().large_image(keys.1).large_text(keys.0))
                    .details(details)
                    .state(state);

                client.set_activity(activity)?;
                log("Updated activity", None);

                sleep();
                xcode_is_running = check_xcode()?
            }
        } else {
            log("Xcode is not running", None)
        }
        sleep()
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn check_xcode() -> Result<bool, Box<dyn std::error::Error>> {
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

fn current_file() -> Result<String, Box<dyn std::error::Error>> {
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

fn current_project() -> Result<String, Box<dyn std::error::Error>> {
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

fn run_osascript(script: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute command");
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

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
fn sleep() {
    thread::sleep(Duration::from_secs(WAIT_TIME))
}
