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

/// Get the current file's name as a String.
///
/// Tries to get the file name from the active source editor document path first
/// (more reliable), falling back to parsing the window title.
pub fn current_file() -> Result<String> {
    // Prefer reading the document path directly — it works even when the window
    // title omits the separator (e.g. project-only titles).
    if let Ok(file) = current_file_from_source_editor() {
        if !file.is_empty() {
            return Ok(file);
        }
    }

    // Fall back to parsing the window title (format: "FileName.swift — ProjectName")
    let raw = run_osascript(
        r#"
        tell application "Xcode"
            return name of windows whose index is 1
        end tell
    "#,
    )?;
    log::debug!("current_file (window title) raw: {:?}", raw);

    // The window title format is "FileName.swift — ProjectName"; take the first part.
    let file = if raw.contains(" — ") {
        raw.split(" — ").collect::<Vec<&str>>()[0].to_string()
    } else {
        raw
    };
    log::debug!("current_file (window title) parsed: {:?}", file);
    Ok(file)
}

/// Get the current file name from the active source editor document path.
///
/// This is more reliable than parsing the window title because it always
/// reflects the file that is actually open in the editor.
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
