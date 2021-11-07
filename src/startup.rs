use crate::utils;
use colored::*;

use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

pub async fn up() -> crate::Result<()> {
    init_fern_logger();
    // init_json_logger();
    dotenv::dotenv().ok();
    log_app_env();
    Ok(())
}

fn init_fern_logger() {
    let colors = ColoredLevelConfig::new();
    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}]{} {}",
                // This will color the log level only, not the whole line. Just a touch.
                colors.color(record.level()),
                chrono::Utc::now().format("[%Y-%m-%d %H:%M:%S]"),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .apply()
        .unwrap();
}

#[allow(dead_code)] // in the future we can turn on json logging via config
fn init_json_logger() {
    let encoder = JsonEncoder::new();

    let stdout: ConsoleAppender = ConsoleAppender::builder().encoder(Box::new(encoder)).build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

fn log_app_env() {
    println!("Environment Variables:");
    get_required_env_names()
        .map(|var| (var, utils::env(var).unwrap_or("<NOT_FOUND>".to_owned())))
        .for_each(|(var, val)| {
            println!("  {}: {}", var.color(Color::BrightBlack), val.color(Color::Green));
        });
    println!();
}

fn get_required_env_names() -> impl Iterator<Item = &'static str> {
    include_str!("../.env.template")
        .lines()
        .filter(|line| !line.starts_with("#") && !line.is_empty())
        .map(|line| line.split("=").take(1))
        .flatten()
}
