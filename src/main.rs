// #![windows_subsystem = "windows"]

mod app;
pub(crate) mod assets;
mod error;
mod platform;
pub(crate) mod state;

use app::Daemon;
pub use error::*;
use iced::Font;
use std::{env, path::PathBuf};

fn main() -> crate::Result<()> {
    let initializer = || {
        let args: Vec<String> = env::args().collect();
        let config_location = match args.len() > 1 {
            true => Some(PathBuf::from(args[1].clone())),
            false => None,
        };
        Daemon::new(config_location)
    };

    iced::daemon(Daemon::title, Daemon::update, Daemon::view)
        .subscription(Daemon::subscription)
        .theme(Daemon::theme)
        .default_font(Font::with_name("Monserrat"))
        .run_with(initializer)?;

    Ok(())
}
