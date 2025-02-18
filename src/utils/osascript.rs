use std::process::Command;

use crate::{Error, Result};

/// Execute an AppleScript command using osascript and returns the output as a String
pub fn run_osascript(script: &str) -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|err| Error::Oascript(err.to_string()))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check if Xcode is running
pub fn check_xcode() -> Result<bool> {
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
pub fn current_file() -> Result<String> {
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
pub fn current_project() -> Result<String> {
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

/// Check if frontmost application is Xcode
pub fn is_xcode_frontmost() -> Result<bool> {
    let frontmost_app = run_osascript(
        r#"
        if frontmost of application "Xcode" is true then
            return "Xcode"
        end if
    "#,
    )?;
    Ok(frontmost_app == "Xcode")
}
