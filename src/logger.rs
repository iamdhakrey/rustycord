use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;
use std::fs::File;
use std::io;

/// Setup the logger with the specified verbosity level
///
/// # Arguments
/// * `verbose` - The verbosity level: "trace", "debug", "info", "warn", "error"
///
/// # Example
/// ```rust
/// use rustcord::logger::setup_logger;
///
/// // Enable debug logging
/// setup_logger("debug".to_string()).expect("Failed to initialize logger");
///
/// // Enable info logging (recommended for production)
/// setup_logger("info".to_string()).expect("Failed to initialize logger");
/// ```
pub fn setup_logger(verbose: String) -> Result<(), Box<dyn std::error::Error>> {
    // Configure the logger

    let colors = ColoredLevelConfig::new()
        .debug(Color::White)
        .error(Color::Red)
        .info(Color::Green)
        .warn(Color::Yellow)
        .trace(Color::Cyan);

    let level = match verbose.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info, // Default to info instead of warn
    };

    let stdout_config = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} :: [{}] :: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                // record.level(),
                message
            ))
        })
        .level(level)
        .chain(io::stdout());

    let log_file = File::create("output.log")?;
    let file_config = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} :: [{}] :: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(log_file);

    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;
    // Dispatch::new().level(level).chain(stdout_config).apply()?;
    Ok(())
}
