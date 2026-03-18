// x-viz cross-platform abstraction layer
// Inspired by RustDesk's src/platform/mod.rs pattern:
// Each platform exports the same public API, selected via #[cfg(target_os)].

use std::path::Path;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

/// Result type for platform operations.
pub type PlatformResult<T> = Result<T, PlatformError>;

/// Platform-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("Failed to set wallpaper: {0}")]
    SetWallpaper(String),

    #[error("Failed to get current wallpaper: {0}")]
    GetWallpaper(String),

    #[error("Unsupported desktop environment: {0}")]
    UnsupportedDesktop(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Ensure PlatformError is serializable for Tauri IPC
impl serde::Serialize for PlatformError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Trait that each platform module must implement.
/// This is the contract — any platform-specific code must provide these.
pub trait WallpaperOps {
    fn set_wallpaper(path: &Path) -> PlatformResult<()>;
    fn get_current_wallpaper() -> PlatformResult<Option<String>>;
}
