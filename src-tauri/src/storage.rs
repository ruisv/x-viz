// Storage management module.
// Handles wallpaper file listing and LRU cleanup.

use crate::bing::BingImage;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents a locally stored wallpaper with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalWallpaper {
    pub title: String,
    pub date: String,
    pub path: String,
    pub copyright: String,
}

/// Ensure the wallpaper storage directory exists.
pub fn ensure_dir(storage_path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(storage_path);
    std::fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create storage dir {:?}: {e}", path))?;
    Ok(path)
}

/// List all wallpaper images in the storage directory, sorted by date (newest first).
pub fn list_wallpapers(storage_path: &str) -> Result<Vec<LocalWallpaper>, String> {
    let dir = PathBuf::from(storage_path);
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut wallpapers: Vec<LocalWallpaper> = Vec::new();

    let entries = std::fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read directory: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if is_image_file(&path) {
            let filename = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // Try to extract date from filename (e.g., "2026-03-17_OHR.xxx")
            let date = extract_date_from_filename(&filename);

            wallpapers.push(LocalWallpaper {
                title: clean_title(&filename),
                date,
                path: path.to_string_lossy().to_string(),
                copyright: String::new(),
            });
        }
    }

    // Sort by date descending (newest first)
    wallpapers.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(wallpapers)
}

/// Build the save path for a Bing image.
///
/// Format: `{storage_path}/{date}_{title}.jpg`
/// Falls back to URL-based filename if the title is empty.
pub fn build_save_path(storage_path: &str, image: &BingImage) -> PathBuf {
    let title = sanitize_filename(&image.title);
    let save_name = if title.is_empty() {
        let fallback = crate::bing::extract_filename(&image.url);
        format!("{}_{}", image.date, fallback)
    } else {
        format!("{}_{}.jpg", image.date, title)
    };
    PathBuf::from(storage_path).join(save_name)
}

/// Check if an image for a given date already exists in the storage directory.
pub fn has_image_for_date(storage_path: &str, date: &str) -> bool {
    let dir = PathBuf::from(storage_path);
    if !dir.exists() {
        return false;
    }
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(date) && is_image_file(&entry.path()) {
                return true;
            }
        }
    }
    false
}

/// Sanitize a string for use as a filename.
fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Clean up wallpapers older than `retention_days`.
/// Returns the number of files deleted.
pub fn cleanup_old(storage_path: &str, retention_days: u32) -> Result<usize, String> {
    let dir = PathBuf::from(storage_path);
    if !dir.exists() {
        return Ok(0);
    }

    let cutoff = chrono::Utc::now()
        - chrono::Duration::days(retention_days as i64);
    let cutoff_str = cutoff.format("%Y-%m-%d").to_string();

    let mut deleted = 0;
    let entries = std::fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read dir: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !is_image_file(&path) {
            continue;
        }

        let filename = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let date = extract_date_from_filename(&filename);
        if !date.is_empty() && date < cutoff_str {
            if let Err(e) = std::fs::remove_file(&path) {
                log::warn!("Failed to delete {:?}: {e}", path);
            } else {
                log::info!("Cleaned up old wallpaper: {:?}", path);
                deleted += 1;
            }
        }
    }

    log::info!(
        "Cleanup complete: {} file(s) deleted (retention: {} days)",
        deleted,
        retention_days
    );
    Ok(deleted)
}

/// Check if a file has an image extension.
fn is_image_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .as_deref(),
        Some("jpg" | "jpeg" | "png" | "webp" | "bmp")
    )
}

/// Extract a date prefix (YYYY-MM-DD) from a filename.
fn extract_date_from_filename(filename: &str) -> String {
    if filename.len() >= 10 && filename.as_bytes()[4] == b'-' && filename.as_bytes()[7] == b'-' {
        filename[..10].to_string()
    } else {
        String::new()
    }
}

/// Clean up a filename into a human-readable title.
fn clean_title(filename: &str) -> String {
    // Remove date prefix and OHR. prefix
    let s = if filename.len() > 11 && filename.starts_with("20") {
        &filename[11..] // Skip "2026-03-17_"
    } else {
        filename
    };
    s.replace("OHR.", "")
        .replace('_', " ")
        .split('.')
        .next()
        .unwrap_or(s)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image_file() {
        assert!(is_image_file(Path::new("test.jpg")));
        assert!(is_image_file(Path::new("test.PNG")));
        assert!(!is_image_file(Path::new("test.txt")));
        assert!(!is_image_file(Path::new("test")));
    }

    #[test]
    fn test_extract_date() {
        assert_eq!(
            extract_date_from_filename("2026-03-17_OHR.Test.jpg"),
            "2026-03-17"
        );
        assert_eq!(extract_date_from_filename("no_date_here"), "");
    }

    #[test]
    fn test_clean_title() {
        assert_eq!(
            clean_title("2026-03-17_OHR.FoxSparrow_EN-US1234_UHD"),
            "FoxSparrow EN-US1234 UHD"
        );
    }
}
