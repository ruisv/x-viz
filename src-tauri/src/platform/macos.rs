// macOS wallpaper implementation
// Uses AppleScript via osascript for maximum compatibility.

use super::{PlatformError, PlatformResult, WallpaperOps};
use std::path::Path;
use std::process::Command;

pub struct Platform;

impl WallpaperOps for Platform {
    fn set_wallpaper(path: &Path) -> PlatformResult<()> {
        let path_str = path
            .to_str()
            .ok_or_else(|| PlatformError::SetWallpaper("Invalid path encoding".into()))?;

        // Use AppleScript to set the wallpaper on all desktops
        let script = format!(
            r#"tell application "System Events"
    tell every desktop
        set picture to POSIX file "{}"
    end tell
end tell"#,
            path_str
        );

        let output = Command::new("osascript")
            .args(["-e", &script])
            .output()
            .map_err(|e| PlatformError::CommandFailed(format!("osascript: {e}")))?;

        if output.status.success() {
            log::info!("Wallpaper set successfully: {}", path_str);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(PlatformError::SetWallpaper(format!(
                "osascript failed: {stderr}"
            )))
        }
    }

    fn get_current_wallpaper() -> PlatformResult<Option<String>> {
        let script = r#"tell application "System Events"
    tell current desktop
        get picture
    end tell
end tell"#;

        let output = Command::new("osascript")
            .args(["-e", script])
            .output()
            .map_err(|e| PlatformError::CommandFailed(format!("osascript: {e}")))?;

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if path.is_empty() {
                Ok(None)
            } else {
                Ok(Some(path))
            }
        } else {
            Ok(None)
        }
    }
}

/// Convenience function — set wallpaper using the platform implementation.
pub fn set_wallpaper(path: &Path) -> PlatformResult<()> {
    Platform::set_wallpaper(path)
}

/// Convenience function — get current wallpaper path.
pub fn get_current_wallpaper() -> PlatformResult<Option<String>> {
    Platform::get_current_wallpaper()
}
