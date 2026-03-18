// Tauri IPC commands — exposed to the frontend via invoke().
// Each #[tauri::command] function maps to a frontend `invoke("command_name", {...})` call.

use crate::{config, platform, scheduler, storage};
use tauri_plugin_dialog::DialogExt;

/// Get the current application configuration.
#[tauri::command]
pub fn get_config() -> config::AppConfig {
    config::load()
}

/// Save updated configuration and sync autostart state.
#[tauri::command]
pub fn save_config(app: tauri::AppHandle, config: config::AppConfig) -> Result<(), String> {
    // Sync autostart with the system
    use tauri_plugin_autostart::ManagerExt;
    let mgr = app.autolaunch();
    if config.auto_start {
        let _ = mgr.enable();
    } else {
        let _ = mgr.disable();
    }

    config::save(&config)
}

/// Fetch wallpapers from Bing and set the latest as the desktop wallpaper.
#[tauri::command]
pub async fn fetch_wallpapers(app: tauri::AppHandle) -> Result<(), String> {
    scheduler::run_update_cycle(&app).await;
    Ok(())
}

/// Set a specific local image as the wallpaper.
#[tauri::command]
pub fn set_wallpaper_now(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }
    platform::set_wallpaper(p).map_err(|e| e.to_string())
}

/// Get the list of locally stored wallpapers for the gallery.
#[tauri::command]
pub fn get_gallery() -> Result<Vec<storage::LocalWallpaper>, String> {
    let cfg = config::load();
    storage::list_wallpapers(&cfg.storage_path)
}

/// Open a native folder picker and return the selected path.
#[tauri::command]
pub async fn choose_storage_path(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let folder = app.dialog().file().blocking_pick_folder();
    Ok(folder.map(|p| p.to_string()))
}

/// Delete a wallpaper file from the gallery.
#[tauri::command]
pub fn delete_wallpaper(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }
    std::fs::remove_file(p).map_err(|e| format!("Failed to delete: {e}"))?;
    log::info!("Deleted wallpaper: {path}");
    Ok(())
}

/// Get basic system information for the settings UI.
#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        #[cfg(target_os = "linux")]
        desktop_env: format!("{:?}", platform::detect_desktop_env()),
        #[cfg(not(target_os = "linux"))]
        desktop_env: "N/A".to_string(),
    }
}

#[derive(serde::Serialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub desktop_env: String,
}
