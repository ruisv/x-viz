// Bing API interaction module
// Fetches daily wallpaper metadata and downloads images.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const BING_API_BASE: &str = "https://www.bing.com/HPImageArchive.aspx";
const BING_BASE_URL: &str = "https://www.bing.com";

/// Metadata for a single Bing daily image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingImage {
    pub title: String,
    pub date: String,
    pub url: String,
    pub url_base: String,
    pub copyright: String,
    pub copyright_link: String,
}

/// Raw API response from Bing.
#[derive(Debug, Deserialize)]
struct BingApiResponse {
    images: Vec<BingApiImage>,
}

#[derive(Debug, Deserialize)]
struct BingApiImage {
    #[serde(default)]
    title: String,
    startdate: String,
    url: String,
    urlbase: String,
    copyright: String,
    #[serde(default)]
    copyrightlink: String,
}

/// Fetch daily images from Bing API.
///
/// - `region`: Market locale (e.g., "en-US", "zh-CN")
/// - `count`: Number of images to fetch (1–8)
pub async fn fetch_daily_images(region: &str, count: u8) -> Result<Vec<BingImage>, String> {
    let count = count.min(8).max(1);
    let url = format!(
        "{}?format=js&idx=0&n={}&mkt={}",
        BING_API_BASE, count, region
    );

    log::info!("Fetching Bing API: {}", url);

    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    let api_response: BingApiResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse API response: {e}"))?;

    let images: Vec<BingImage> = api_response
        .images
        .into_iter()
        .map(|img| {
            let full_url = format!("{}{}", BING_BASE_URL, img.url);
            BingImage {
                title: img.title,
                date: format_date(&img.startdate),
                url: full_url,
                url_base: img.urlbase,
                copyright: img.copyright,
                copyright_link: img.copyrightlink,
            }
        })
        .collect();

    log::info!("Fetched {} images", images.len());
    Ok(images)
}

/// Upgrade a standard Bing image URL to UHD (4K) resolution.
///
/// Transforms: `_1920x1080.jpg` → `_UHD.jpg`
pub fn upgrade_to_uhd(url: &str) -> String {
    // Pattern: /th?id=OHR.xxx_1920x1080.jpg&...
    if url.contains("_1920x1080") {
        url.replace("_1920x1080", "_UHD")
    } else {
        // Try to construct UHD URL from urlbase
        url.to_string()
    }
}

/// Download an image from a URL to the specified path.
pub async fn download_image(url: &str, save_path: &Path) -> Result<PathBuf, String> {
    log::info!("Downloading: {} → {:?}", url, save_path);

    // Create parent directories if they don't exist
    if let Some(parent) = save_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {e}"))?;
    }

    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("Download failed: {e}"))?;

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    std::fs::write(save_path, &bytes)
        .map_err(|e| format!("Failed to write file: {e}"))?;

    log::info!("Downloaded {} bytes to {:?}", bytes.len(), save_path);
    Ok(save_path.to_path_buf())
}

/// Extract a clean filename from a Bing image URL.
///
/// Input:  `/th?id=OHR.FoxSparrow_EN-US1234_UHD.jpg&rf=...`
/// Output: `OHR.FoxSparrow_EN-US1234_UHD.jpg`
pub fn extract_filename(url: &str) -> String {
    url.split("id=")
        .nth(1)
        .unwrap_or("wallpaper.jpg")
        .split('&')
        .next()
        .unwrap_or("wallpaper.jpg")
        .to_string()
}

/// Format a Bing date string "20260317" into "2026-03-17".
fn format_date(raw: &str) -> String {
    NaiveDate::parse_from_str(raw, "%Y%m%d")
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|_| raw.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_to_uhd() {
        let url = "https://www.bing.com/th?id=OHR.Test_EN-US123_1920x1080.jpg&rf=abc";
        assert!(upgrade_to_uhd(url).contains("_UHD"));
        assert!(!upgrade_to_uhd(url).contains("_1920x1080"));
    }

    #[test]
    fn test_extract_filename() {
        let url = "/th?id=OHR.FoxSparrow_EN-US1234_UHD.jpg&rf=123";
        assert_eq!(extract_filename(url), "OHR.FoxSparrow_EN-US1234_UHD.jpg");
    }

    #[test]
    fn test_format_date() {
        assert_eq!(format_date("20260317"), "2026-03-17");
        assert_eq!(format_date("invalid"), "invalid");
    }
}
