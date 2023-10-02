use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};
use std::{process::Command, thread, time::Duration};

const WAIT_TIME: u64 = 5;

fn main() {
    loop {
        if let Err(err) = discord_rpc() {
            println!("Error: {}", err);
            thread::sleep(Duration::from_secs(WAIT_TIME));
        }
    }
}

fn discord_rpc() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1158013054898950185")?;

    loop {
        if client.connect().is_ok() {
            thread::sleep(Duration::from_secs(WAIT_TIME));
            break;
        }
    }

    loop {
        let connection = client.connect();

        if connection.is_err() {
            thread::sleep(Duration::from_secs(WAIT_TIME));
            client.reconnect()?;
            continue;
        }

        if connection.is_ok() {
            println!("Connected to Discord");
            let mut xcode_is_running = check_xcode()?;

            while xcode_is_running {
                let file = current_file()?;
                let project = current_project()?;
                let file_extension = (file.split('.').collect::<Vec<&str>>().last().unwrap_or(&""))
                    .trim()
                    .to_string();

                let state = format!("Working on {}", file);
                let details = format!("In {}", project);

                let keys = match file_extension.as_str() {
                    "swift" => ("Swift".to_string(), "swift".to_string()),
                    "rb" => ("Ruby".to_string(), "ruby".to_string()),
                    "cpp" | "cp" | "cxx" => ("C++".to_string(), "cpp".to_string()),
                    "java" => ("Java".to_string(), "java".to_string()),
                    _ => ("Xcode".to_string(), "xcode".to_string()),
                };

                let activity = Activity::new()
                    .state(&state)
                    .details(&details)
                    .assets(Assets::new().large_image(&keys.1).large_text(&keys.0));
                client.set_activity(activity)?;

                thread::sleep(Duration::from_secs(WAIT_TIME));

                xcode_is_running = check_xcode()?;
            }
        }

        thread::sleep(Duration::from_secs(30));
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
