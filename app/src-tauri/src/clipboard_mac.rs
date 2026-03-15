use std::path::Path;

/// Copy a file to the clipboard by having Finder do it — the only method
/// that produces clipboard data all macOS apps (including sandboxed ones) accept.
pub fn copy_file_to_clipboard(path: &Path) -> Result<(), String> {
    let abs = path
        .canonicalize()
        .map_err(|e| format!("Cannot resolve path: {}", e))?;
    let path_str = abs.to_str().ok_or("Path is not valid UTF-8")?;

    let script = format!(
        r#"
tell application "Finder"
    set theFile to POSIX file "{}" as alias
    reveal theFile
    activate
    select theFile
end tell
delay 0.3
tell application "System Events"
    keystroke "c" using command down
end tell
delay 0.2
"#,
        path_str
    );

    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to run osascript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Finder copy failed: {}", stderr));
    }

    Ok(())
}
