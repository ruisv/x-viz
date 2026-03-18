// Application configuration persistence module.
// Stores settings as JSON in the platform-specific app data directory.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILENAME: &str = "config.json";

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Market region for Bing API (e.g., "en-US", "zh-CN").
    pub region: String,

    /// Directory path for storing downloaded wallpapers.
    pub storage_path: String,

    /// Number of days to retain wallpapers before cleanup.
    pub retention_days: u32,

    /// Whether the app should auto-start on login.
    pub auto_start: bool,

    /// Update interval: "daily", "hourly", or "manual".
    pub update_interval: String,

    /// Resolution preference: "UHD" or "1920x1080".
    pub resolution: String,

    /// Number of wallpapers to fetch per update (1–8).
    #[serde(default = "default_fetch_count")]
    pub fetch_count: u8,

    /// Hide window on startup (tray-only mode).
    #[serde(default)]
    pub silent_start: bool,

    /// UI language: "zh-CN" or "en-US". Default from system locale.
    #[serde(default = "detect_system_language")]
    pub language: String,

    /// Theme: "auto", "light", or "dark". Default: "auto" (follow system).
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_fetch_count() -> u8 {
    1
}

fn default_theme() -> String {
    "auto".into()
}

fn detect_system_language() -> String {
    if let Some(locale) = sys_locale::get_locale() {
        let l = locale.to_lowercase();
        if l.starts_with("zh") {
            return "zh-CN".into();
        }
    }
    "en-US".into()
}

/// Detect the system locale and map it to a Bing market code.
fn detect_system_region() -> String {
    if let Some(locale) = sys_locale::get_locale() {
        // sys_locale returns e.g. "zh-Hans-CN", "en-US", "ja-JP"
        // Map common patterns to Bing market codes
        let locale_lower = locale.to_lowercase();
        if locale_lower.starts_with("zh-hans") || locale_lower == "zh-cn" {
            return "zh-CN".into();
        }
        if locale_lower.starts_with("zh-hant") || locale_lower == "zh-tw" || locale_lower == "zh-hk" {
            return "zh-TW".into();
        }
        if locale_lower.starts_with("ja") {
            return "ja-JP".into();
        }
        if locale_lower.starts_with("de") {
            return "de-DE".into();
        }
        if locale_lower.starts_with("fr") {
            return "fr-FR".into();
        }
        if locale_lower.starts_with("en-gb") {
            return "en-GB".into();
        }
        if locale_lower.starts_with("en-au") {
            return "en-AU".into();
        }
        if locale_lower.starts_with("en-in") {
            return "en-IN".into();
        }
        if locale_lower.starts_with("en") {
            return "en-US".into();
        }
        // Fallback: try to use as-is if it looks like a market code (xx-XX)
        if locale.len() >= 5 && locale.as_bytes()[2] == b'-' {
            return locale[..5].to_string();
        }
    }
    "en-US".into()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            region: detect_system_region(),
            storage_path: default_storage_path()
                .to_string_lossy()
                .to_string(),
            retention_days: 30,
            auto_start: false,
            update_interval: "daily".into(),
            resolution: "UHD".into(),
            fetch_count: 1,
            silent_start: false,
            language: detect_system_language(),
            theme: "auto".into(),
        }
    }
}

/// Returns true if this is the first launch (no config file on disk).
pub fn is_first_run() -> bool {
    !config_file_path().exists()
}

/// Get the platform-specific configuration directory.
/// e.g., ~/Library/Application Support/com.ruis.x-viz/ on macOS.
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.ruis.x-viz")
}

/// Get the default wallpaper storage path.
/// e.g., ~/Pictures/x-viz/ on macOS/Linux, ~\Pictures\x-viz\ on Windows.
pub fn default_storage_path() -> PathBuf {
    dirs::picture_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
        .join("x-viz")
}

/// Full path to the config file.
fn config_file_path() -> PathBuf {
    config_dir().join(CONFIG_FILENAME)
}

/// Load configuration from disk. Returns default if file doesn't exist.
pub fn load() -> AppConfig {
    let path = config_file_path();
    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(cfg) => {
                    log::info!("Loaded config from {:?}", path);
                    return cfg;
                }
                Err(e) => {
                    log::warn!("Failed to parse config: {e}, using defaults");
                }
            },
            Err(e) => {
                log::warn!("Failed to read config file: {e}, using defaults");
            }
        }
    } else {
        log::info!("No config file found, using defaults");
    }
    AppConfig::default()
}

/// Save configuration to disk.
pub fn save(config: &AppConfig) -> Result<(), String> {
    let path = config_file_path();

    // Ensure the config directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config dir: {e}"))?;
    }

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {e}"))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("Failed to write config file: {e}"))?;

    log::info!("Config saved to {:?}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.region, "en-US");
        assert_eq!(cfg.retention_days, 30);
        assert_eq!(cfg.resolution, "UHD");
        assert!(!cfg.auto_start);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let cfg = AppConfig::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let parsed: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.region, cfg.region);
        assert_eq!(parsed.retention_days, cfg.retention_days);
    }
}
