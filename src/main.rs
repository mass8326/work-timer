#![windows_subsystem = "windows"]

mod app;
mod error;
mod logger;
mod platform;

pub(crate) mod assets;
pub(crate) mod state;

use app::{Daemon, Message};
use iced::{Font, Task};
use std::{env, path::PathBuf};

pub use error::*;

fn main() -> crate::Result<()> {
    iced::daemon(Daemon::title, Daemon::update, Daemon::view)
        .subscription(Daemon::subscription)
        .theme(Daemon::theme)
        .default_font(Font::with_name("Monserrat"))
        .run_with(init)?;
    Ok(())
}

fn init() -> (Daemon, Task<Message>) {
    logger::init();
    let args: Vec<String> = env::args().collect();
    let config_location = match args.len() > 1 {
        true => Some(PathBuf::from(args[1].clone())),
        false => None,
    };
    Daemon::new(config_location)
}
