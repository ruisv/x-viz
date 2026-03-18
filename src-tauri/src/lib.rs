// x-viz — Bing daily wallpaper desktop application
// Main library entry point: module declarations, plugin registration, and app setup.

mod bing;
mod commands;
mod config;
mod logger;
mod platform;
mod scheduler;
mod storage;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    Emitter, Manager, WebviewWindowBuilder, WindowEvent,
};

/// Hide app from Dock (macOS only).
fn hide_dock(app: &tauri::AppHandle) {
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_dock_visibility(false);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
    }
}

/// Show app in Dock (macOS only).
fn show_dock(app: &tauri::AppHandle) {
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_dock_visibility(true);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
    }
}

/// Show the main window, recreating the WebView if it was destroyed.
fn show_main_window(app: &tauri::AppHandle) {
    show_dock(app);
    if let Some(win) = app.get_webview_window("main") {
        #[cfg(target_os = "macos")]
        let _ = app.show();
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    } else {
        // WebView was destroyed — recreate it.
        let builder = WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
            .title("x-viz")
            .inner_size(800.0, 560.0)
            .min_inner_size(640.0, 480.0)
            .center()
            .decorations(true);

        #[cfg(target_os = "macos")]
        let builder = builder
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true);

        match builder.build() {
            Ok(win) => {
                #[cfg(target_os = "macos")]
                let _ = app.show();
                let _ = win.set_focus();
                // Re-attach close → destroy handler
                let app_handle = app.clone();
                win.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        if let Some(w) = app_handle.get_webview_window("main") {
                            let _ = w.destroy();
                        }
                        hide_dock(&app_handle);
                    }
                });
            }
            Err(e) => log::error!("Failed to recreate main window: {e}"),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init(&config::config_dir());

    log::info!("Starting x-viz v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::fetch_wallpapers,
            commands::set_wallpaper_now,
            commands::get_gallery,
            commands::delete_wallpaper,
            commands::choose_storage_path,
            commands::get_system_info,
        ])
        .setup(|app| {
            // --- Build menu items ---
            let update_item =
                MenuItemBuilder::with_id("update", "获取最新壁纸").build(app)?;
            let settings_item =
                MenuItemBuilder::with_id("settings", "打开主窗口…").build(app)?;
            let open_folder_item =
                MenuItemBuilder::with_id("open_folder", "打开壁纸文件夹…").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出 x-viz").build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&update_item)
                .separator()
                .item(&settings_item)
                .item(&open_folder_item)
                .separator()
                .item(&quit_item)
                .build()?;

            // --- Get the tray icon created from tauri.conf.json (id: "main") ---
            let tray = app
                .tray_by_id("main")
                .expect("tray icon 'main' not found — check tauri.conf.json trayIcon.id");

            tray.set_menu(Some(menu))?;
            tray.on_menu_event(move |app, event| match event.id().as_ref() {
                "update" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        scheduler::run_update_cycle(&app_handle).await;
                        if let Some(win) = app_handle.get_webview_window("main") {
                            let _ = win.emit("wallpapers-updated", ());
                        }
                    });
                }
                "settings" => {
                    show_main_window(app);
                }
                "open_folder" => {
                    let cfg = config::load();
                    let _ = opener::open(&cfg.storage_path);
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            });

            // --- First launch: show window; otherwise respect silent_start ---
            let first_run = config::is_first_run();
            let cfg = config::load();
            if first_run || !cfg.silent_start {
                show_main_window(app.handle());
            }
            // First launch: save default config (with silent_start=true) so
            // next launch will be silent.
            if first_run {
                let _ = config::save(&cfg);
            }

            // --- Sync autostart state with the plugin ---
            {
                use tauri_plugin_autostart::ManagerExt;
                let mgr = app.autolaunch();
                if cfg.auto_start {
                    let _ = mgr.enable();
                } else {
                    let _ = mgr.disable();
                }
            }

            // --- Close → destroy WebView to free memory, hide from Dock ---
            let main_window = app.get_webview_window("main").unwrap();
            let app_handle = app.handle().clone();
            main_window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Some(w) = app_handle.get_webview_window("main") {
                        let _ = w.destroy();
                    }
                    hide_dock(&app_handle);
                }
            });

            // --- Start background scheduler ---
            let sched = scheduler::Scheduler::new(app.handle().clone());
            tauri::async_runtime::spawn(async move {
                sched.start().await;
            });

            log::info!("App setup complete");
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // Prevent app from exiting when all windows are closed,
            // but allow explicit exit (e.g. tray "quit" → app.exit(0)).
            if let tauri::RunEvent::ExitRequested { code, api, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
