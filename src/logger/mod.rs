use crate::constants;
use crate::utils;
use chrono::Local;
use colored::*;
use fern::Dispatch;
use log::{Level, LevelFilter};

fn level_color(level: Level) -> Color {
    match level {
        Level::Trace => Color::Magenta,
        Level::Debug => Color::Cyan,
        Level::Info => Color::Green,
        Level::Warn => Color::Yellow,
        Level::Error => Color::Red,
    }
}

pub fn init_logger() {
    Dispatch::new()
        .format(|out, message, record| {
            let color = level_color(record.level());
            out.finish(format_args!(
                "{}: {}",
                format!("[{}] [{}]", Local::now().format("%Y-%m-%d %H:%M:%S"), record.level()).color(color),
                message
            ))
        })
        .level(
            utils::env(constants::env::LOG_LEVEL)
                .and_then(|level| level.parse::<LevelFilter>().ok())
                .unwrap_or(LevelFilter::Info),
        )
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

#[macro_export]
macro_rules! error {
  ($($args:tt)*) => {{
    log::error!($($args)*);
  }}
}

#[macro_export]
macro_rules! warn {
  ($($args:tt)*) => {{
    log::warn!($($args)*);
  }}
}

#[macro_export]
macro_rules! info {
  ($($args:tt)*) => {{
    log::info!($($args)*);
  }}
}

#[macro_export]
macro_rules! debug {
  ($($args:tt)*) => {{
    log::debug!($($args)*);
  }}
}

#[macro_export]
macro_rules! trace {
  ($($args:tt)*) => {{
    log::trace!($($args)*);
  }}
}
