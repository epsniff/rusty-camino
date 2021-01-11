use crate::{logger, utils};
use colored::*;

pub async fn up() -> crate::Result<()> {
    dotenv::dotenv().ok();
    logger::init_logger();
    log_app_env();
    Ok(())
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
