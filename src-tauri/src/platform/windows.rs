// Windows wallpaper implementation
// Uses SystemParametersInfoW from windows-sys for native wallpaper setting.

use super::{PlatformError, PlatformResult, WallpaperOps};
use std::path::Path;

pub struct Platform;

impl WallpaperOps for Platform {
    fn set_wallpaper(path: &Path) -> PlatformResult<()> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE,
        };

        let path_str = path
            .to_str()
            .ok_or_else(|| PlatformError::SetWallpaper("Invalid path encoding".into()))?;

        // Convert to wide string (null-terminated UTF-16)
        let wide: Vec<u16> = OsStr::new(path_str)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let result = unsafe {
            SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                wide.as_ptr() as *mut _,
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            )
        };

        if result != 0 {
            log::info!("Wallpaper set successfully: {}", path_str);
            Ok(())
        } else {
            Err(PlatformError::SetWallpaper(
                "SystemParametersInfoW returned 0".into(),
            ))
        }
    }

    fn get_current_wallpaper() -> PlatformResult<Option<String>> {
        // Read from registry: HKCU\Control Panel\Desktop\Wallpaper
        use std::process::Command;

        let output = Command::new("reg")
            .args([
                "query",
                r"HKCU\Control Panel\Desktop",
                "/v",
                "Wallpaper",
            ])
            .output()
            .map_err(|e| PlatformError::CommandFailed(format!("reg query: {e}")))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse the REG_SZ value from the output
            if let Some(line) = stdout.lines().find(|l| l.contains("Wallpaper")) {
                if let Some(val) = line.split("REG_SZ").nth(1) {
                    let path = val.trim().to_string();
                    if path.is_empty() {
                        return Ok(None);
                    }
                    return Ok(Some(path));
                }
            }
        }
        Ok(None)
    }
}

pub fn set_wallpaper(path: &Path) -> PlatformResult<()> {
    Platform::set_wallpaper(path)
}

pub fn get_current_wallpaper() -> PlatformResult<Option<String>> {
    Platform::get_current_wallpaper()
}
