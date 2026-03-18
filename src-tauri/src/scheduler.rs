// Background scheduler module.
// Uses wall-clock time to decide when to run updates — correctly handles
// system sleep/wake (unlike interval-based timers that pause during sleep).

use crate::{bing, config, platform, storage};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

/// Unix timestamp of the last successful wallpaper update.
static LAST_UPDATE_TS: AtomicU64 = AtomicU64::new(0);

/// Payload emitted as the `update-result` event to the frontend.
#[derive(Clone, serde::Serialize)]
pub struct UpdateResult {
    pub success: bool,
    pub message: String,
}

pub struct Scheduler {
    app: AppHandle,
    running: Arc<Mutex<bool>>,
}

impl Scheduler {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start the background scheduler loop.
    ///
    /// Runs an update immediately on start, then polls every 5 minutes
    /// using wall-clock time to decide whether the configured interval has
    /// elapsed.  This correctly handles system sleep/wake.
    pub async fn start(&self) {
        let mut running = self.running.lock().await;
        if *running {
            log::info!("Scheduler already running");
            return;
        }
        *running = true;
        drop(running);

        let running_flag = self.running.clone();
        let app = self.app.clone();

        tokio::spawn(async move {
            log::info!("Scheduler started");

            // Run once immediately on start.
            run_update_cycle(&app).await;

            // Poll every 5 minutes. Using a short poll interval instead of a
            // long sleep means we notice overdue updates quickly after a
            // system wake-from-sleep.
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;

                if !*running_flag.lock().await {
                    break;
                }

                let cfg = config::load();
                if cfg.update_interval == "manual" {
                    continue;
                }

                let interval_secs: u64 = match cfg.update_interval.as_str() {
                    "hourly" => 3600,
                    _ => 86400, // daily
                };

                let now = chrono::Utc::now().timestamp() as u64;
                let last = LAST_UPDATE_TS.load(Ordering::Relaxed);

                if now.saturating_sub(last) >= interval_secs {
                    run_update_cycle(&app).await;
                }
            }

            log::info!("Scheduler stopped");
        });
    }

    pub async fn stop(&self) {
        *self.running.lock().await = false;
    }
}

/// Run one full update cycle: fetch → download → set wallpaper → cleanup.
///
/// Emits an `update-result` event to the frontend on both success and failure.
/// On success, also emits `wallpapers-updated` to trigger a gallery refresh.
pub async fn run_update_cycle(app: &AppHandle) {
    match run_update_inner(app).await {
        Ok(title) => {
            LAST_UPDATE_TS.store(chrono::Utc::now().timestamp() as u64, Ordering::Relaxed);
            let msg = format!("壁纸已更新：{title}");
            log::info!("{msg}");
            let _ = app.emit("update-result", UpdateResult { success: true, message: msg });
            let _ = app.emit("wallpapers-updated", ());
        }
        Err(e) => {
            log::error!("Update cycle failed: {e}");
            let _ = app.emit(
                "update-result",
                UpdateResult { success: false, message: e },
            );
        }
    }
}

/// Inner update logic. Returns the wallpaper title on success.
async fn run_update_inner(app: &AppHandle) -> Result<String, String> {
    let cfg = config::load();
    log::info!(
        "Running update cycle (region: {}, resolution: {})",
        cfg.region,
        cfg.resolution
    );

    // 1. Fetch images from Bing (user-configured count, 1–8)
    let count = cfg.fetch_count.max(1).min(8);
    let images = bing::fetch_daily_images(&cfg.region, count)
        .await
        .map_err(|e| format!("获取 Bing 图片失败：{e}"))?;

    if images.is_empty() {
        return Err("Bing API 未返回图片".to_string());
    }

    // 2. Download images, skip dates already on disk
    storage::ensure_dir(&cfg.storage_path)
        .map_err(|e| format!("创建存储目录失败：{e}"))?;

    let mut latest_path = None;
    let mut latest_title = String::new();

    for image in &images {
        // Skip if we already have an image for this date
        let save_path = storage::build_save_path(&cfg.storage_path, image);
        if save_path.exists() || storage::has_image_for_date(&cfg.storage_path, &image.date) {
            log::info!("Already have image for {}: skipping", image.date);
            if latest_path.is_none() {
                // Still track the latest for wallpaper setting
                if save_path.exists() {
                    latest_path = Some(save_path);
                }
                latest_title = image.title.clone();
            }
            continue;
        }

        let download_url = if cfg.resolution == "UHD" {
            bing::upgrade_to_uhd(&image.url)
        } else {
            image.url.clone()
        };

        match bing::download_image(&download_url, &save_path).await {
            Ok(_) => {
                if latest_path.is_none() {
                    latest_path = Some(save_path);
                    latest_title = image.title.clone();
                }
            }
            Err(e) => {
                log::warn!("Failed to download {}: {e}", image.title);
            }
        }
    }

    // 3. Set the latest image as desktop wallpaper
    if let Some(path) = &latest_path {
        platform::set_wallpaper(path)
            .map_err(|e| format!("设置壁纸失败：{e}"))?;
    }

    // 4. Cleanup old files
    if let Err(e) = storage::cleanup_old(&cfg.storage_path, cfg.retention_days) {
        log::warn!("Cleanup failed: {e}");
    }

    let _ = app;

    Ok(latest_title)
}
