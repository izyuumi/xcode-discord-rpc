use chrono::Local;
use clap::{Arg, Command as ClapCommand};
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

fn main() {
    // Parse command-line arguments
    let matches = ClapCommand::new("Xcode Discord RPC")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Displays Xcode status on Discord")
        .arg(
            Arg::new("show_file")
                .short('f')
                .long("show-file")
                .num_args(0)
                .help("Show current file"),
        )
        .arg(
            Arg::new("show_project")
                .short('p')
                .long("show-project")
                .num_args(0)
                .help("Show current project"),
        )
        .get_matches();

    let show_file = matches.get_flag("show_file");
    let show_project = matches.get_flag("show_project");

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
                let file = if show_file {
                    current_file()?
                } else {
                    String::from("")
                };
                let project = if show_project {
                    current_project()?
                } else {
                    String::from("")
                };

                if !project_before.eq(&project) {
                    started_at = Timestamps::new().start(current_time());
                    project_before = project.clone();
                }

                // Declare `details` and `state` as `String` types before the activity block
                let details;
                let state;

                let activity = if show_file || show_project {
                    details = if show_file && !file.is_empty() {
                        format!("Working on {}", file)
                    } else {
                        String::from("Working...")
                    };

                    state = if show_project && !project.is_empty() {
                        format!("in {}", project)
                    } else {
                        String::from("in Project")
                    };

                    Activity::new()
                        .details(&details)
                        .state(&state)
                        .assets(Assets::new().large_image("xcode").large_text("Xcode"))
                        .timestamps(started_at.clone())
                } else {
                    // If both are hidden, only show Xcode icon and text
                    Activity::new()
                        .assets(Assets::new().large_image("xcode").large_text("Xcode"))
                        .timestamps(started_at.clone())
                };

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

fn log(message: &str, error: Option<&str>) {
    let date_format = Local::now().format("%Y-%m-%d %H:%M:%S");
    match error {
        Some(error) => eprintln!("{}: {} (Error: {})", date_format, message, error),
        None => println!("{}: {}", date_format, message),
    }
}

fn sleep() {
    thread::sleep(Duration::from_secs(WAIT_TIME))
}
