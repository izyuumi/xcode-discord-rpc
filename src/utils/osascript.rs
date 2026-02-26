use std::process::Command;

use crate::{Error, Result};

/// Execute an AppleScript command using osascript and returns the output as a String
pub fn run_osascript(script: &str) -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|err| Error::Oascript(format!("failed to spawn osascript: {err}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let code = output.status.code().unwrap_or(-1);
        log::debug!("osascript failed (exit {}): {}", code, stderr);
        return Err(Error::Oascript(format!(
            "osascript exited with code {code}: {stderr}"
        )));
    }

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
    let raw = run_osascript(
        r#"
        tell application "Xcode"
            return name of windows whose index is 1
        end tell
    "#,
    )?;
    log::debug!("current_file raw: {:?}", raw);

    let file = if raw.contains(" — ") {
        raw.split(" — ").collect::<Vec<&str>>()[1].to_string()
    } else {
        raw
    };
    log::debug!("current_file parsed: {:?}", file);
    Ok(file)
}

/// Get the current file from the source editor document path (more reliable than window title parsing)
pub fn current_file_from_source_editor() -> Result<String> {
    let raw = run_osascript(
        r#"
        tell application "Xcode"
            set doc to front document
            return path of doc
        end tell
    "#,
    )?;
    log::debug!("current_file_from_source_editor raw: {:?}", raw);

    // Extract just the filename from the full path
    let file = raw
        .rsplit('/')
        .next()
        .unwrap_or(&raw)
        .to_string();
    log::debug!("current_file_from_source_editor parsed: {:?}", file);
    Ok(file)
}

/// Get the current project's name as a String
pub fn current_project() -> Result<String> {
    let raw = run_osascript(
        r#"
        tell application "Xcode"
            return active workspace document
        end tell
    "#,
    )?;
    log::debug!("current_project raw: {:?}", raw);

    let project = if raw == "missing value" {
        String::new()
    } else if raw.starts_with("workspace document ") {
        raw.replace("workspace document ", "")
    } else {
        raw
    };
    log::debug!("current_project parsed: {:?}", project);
    Ok(project)
}

/// Get the filesystem path of the currently active Xcode workspace/project document.
///
/// Returns an empty string when no workspace is open or the AppleScript call
/// fails (e.g. Xcode is not responding).
pub fn current_project_path() -> Result<String> {
    let raw = run_osascript(
        r#"
        tell application "Xcode"
            set doc to active workspace document
            if doc is missing value then
                return ""
            end if
            return path of doc
        end tell
    "#,
    )
    .unwrap_or_default();
    log::debug!("current_project_path raw: {:?}", raw);
    Ok(raw)
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
