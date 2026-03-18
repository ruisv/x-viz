// File + stderr logging initialization.
// Uses fern to write formatted log entries to both stderr and a persistent log file.

use std::path::Path;

/// Initialize the logger. Writes to stderr and to `{log_dir}/x-viz.log`.
/// Safe to call only once — subsequent calls are silently ignored.
pub fn init(log_dir: &Path) {
    let _ = std::fs::create_dir_all(log_dir);
    let log_path = log_dir.join("x-viz.log");

    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stderr());

    match fern::log_file(&log_path) {
        Ok(file) => dispatch = dispatch.chain(file),
        Err(e) => eprintln!("Failed to open log file {log_path:?}: {e}"),
    }

    // Ignore error if a logger is already set (e.g., in tests)
    let _ = dispatch.apply();

    log::info!("Logger initialized — log file: {log_path:?}");
}
