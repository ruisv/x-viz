// Linux wallpaper implementation
// Detects desktop environment and uses the appropriate command.
// Follows RustDesk's pattern of env var detection for desktop environments.

use super::{PlatformError, PlatformResult, WallpaperOps};
use std::path::Path;
use std::process::Command;

/// Known Linux desktop environments.
#[derive(Debug, Clone, PartialEq)]
pub enum DesktopEnv {
    Gnome,
    Kde,
    Xfce,
    Sway,
    Hyprland,
    Cinnamon,
    Mate,
    Unknown(String),
}

/// Detect the current desktop environment from environment variables.
pub fn detect_desktop_env() -> DesktopEnv {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let desktop_lower = desktop.to_lowercase();

    if desktop_lower.contains("gnome") || desktop_lower.contains("unity") {
        DesktopEnv::Gnome
    } else if desktop_lower.contains("kde") {
        DesktopEnv::Kde
    } else if desktop_lower.contains("xfce") {
        DesktopEnv::Xfce
    } else if desktop_lower.contains("sway") {
        DesktopEnv::Sway
    } else if desktop_lower.contains("hyprland") {
        DesktopEnv::Hyprland
    } else if desktop_lower.contains("cinnamon") {
        DesktopEnv::Cinnamon
    } else if desktop_lower.contains("mate") {
        DesktopEnv::Mate
    } else if desktop.is_empty() {
        // Headless or unknown — check $DISPLAY / $WAYLAND_DISPLAY
        if std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok() {
            DesktopEnv::Unknown("unknown-graphical".into())
        } else {
            DesktopEnv::Unknown("headless".into())
        }
    } else {
        DesktopEnv::Unknown(desktop)
    }
}

pub struct Platform;

impl WallpaperOps for Platform {
    fn set_wallpaper(path: &Path) -> PlatformResult<()> {
        let path_str = path
            .to_str()
            .ok_or_else(|| PlatformError::SetWallpaper("Invalid path encoding".into()))?;

        let de = detect_desktop_env();
        log::info!("Detected desktop environment: {:?}", de);

        match de {
            DesktopEnv::Gnome | DesktopEnv::Cinnamon => {
                let uri = format!("file://{}", path_str);
                run_cmd("gsettings", &[
                    "set",
                    "org.gnome.desktop.background",
                    "picture-uri",
                    &uri,
                ])?;
                // Also set for dark mode (GNOME 42+)
                let _ = run_cmd("gsettings", &[
                    "set",
                    "org.gnome.desktop.background",
                    "picture-uri-dark",
                    &uri,
                ]);
                Ok(())
            }
            DesktopEnv::Kde => {
                // KDE Plasma 5.17+
                run_cmd("plasma-apply-wallpaperimage", &[path_str])
            }
            DesktopEnv::Xfce => {
                run_cmd("xfconf-query", &[
                    "-c", "xfce4-desktop",
                    "-p", "/backdrop/screen0/monitor0/workspace0/last-image",
                    "-s", path_str,
                ])
            }
            DesktopEnv::Mate => {
                run_cmd("gsettings", &[
                    "set",
                    "org.mate.background",
                    "picture-filename",
                    path_str,
                ])
            }
            DesktopEnv::Sway | DesktopEnv::Hyprland => {
                // swaybg should be running; we can use swaymsg or direct swaybg
                run_cmd("swaybg", &["-i", path_str, "-m", "fill"])
            }
            DesktopEnv::Unknown(ref name) => {
                log::warn!("Unsupported desktop environment: {}", name);
                Err(PlatformError::UnsupportedDesktop(name.clone()))
            }
        }
    }

    fn get_current_wallpaper() -> PlatformResult<Option<String>> {
        let de = detect_desktop_env();
        match de {
            DesktopEnv::Gnome | DesktopEnv::Cinnamon => {
                let output = Command::new("gsettings")
                    .args(["get", "org.gnome.desktop.background", "picture-uri"])
                    .output()
                    .map_err(|e| PlatformError::CommandFailed(e.to_string()))?;

                if output.status.success() {
                    let val = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .trim_matches('\'')
                        .replace("file://", "")
                        .to_string();
                    Ok(if val.is_empty() { None } else { Some(val) })
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

fn run_cmd(program: &str, args: &[&str]) -> PlatformResult<()> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| PlatformError::CommandFailed(format!("{program}: {e}")))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(PlatformError::CommandFailed(format!(
            "{program} failed: {stderr}"
        )))
    }
}

pub fn set_wallpaper(path: &Path) -> PlatformResult<()> {
    Platform::set_wallpaper(path)
}

pub fn get_current_wallpaper() -> PlatformResult<Option<String>> {
    Platform::get_current_wallpaper()
}
