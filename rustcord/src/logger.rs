use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;
use std::fs::File;
use std::io;

pub fn setup_logger(vervose: String) -> Result<(), Box<dyn std::error::Error>> {
    // Configure the logger

    let colors = ColoredLevelConfig::new()
        .debug(Color::White)
        .error(Color::Red)
        .info(Color::Green)
        .warn(Color::Yellow)
        .trace(Color::Cyan);

    let level = match vervose.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Warn,
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
