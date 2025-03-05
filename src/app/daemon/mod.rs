mod message;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::assets::fonts::FONT_MONSERRAT;
use crate::state::Config;
use crate::state::Whitelist;

use iced::font;
use iced::{
    window::{self, Level},
    Element, Subscription, Task, Theme,
};
use rust_decimal::{prelude::*, Decimal, RoundingStrategy};

use super::settings::Settings;
use super::timer::Timer;

pub use message::*;

#[derive(Debug)]
pub struct Daemon {
    project_file: Option<PathBuf>,
    settings_id: Option<window::Id>,
    whitelist: Arc<Mutex<Whitelist>>,
    timer: Timer,
    settings: Settings,
    theme: Theme,
}

impl Daemon {
    pub fn new(project_file: Option<PathBuf>) -> (Self, Task<Message>) {
        let (config, fallback) = Config::load_or_fallback(project_file.as_ref());
        let whitelist = Arc::new(Mutex::new(config.whitelist.clone().unwrap_or_default()));
        let settings = Settings::new(whitelist.clone());
        let (timer, task) = Timer::new(&config, &whitelist);
        (
            Self {
                project_file: project_file.or(fallback),
                whitelist,
                timer,
                settings,
                theme: match dark_light::detect() {
                    Ok(dark_light::Mode::Light) => Theme::Light,
                    _ => Theme::Dark,
                },
                settings_id: None,
            },
            Task::batch(vec![font::load(FONT_MONSERRAT), task.discard()]).discard(),
        )
    }

    pub fn title(&self, window_id: window::Id) -> String {
        if let Some(settings_id) = self.settings_id {
            if settings_id == window_id {
                return "WorkTimer Settings".to_owned();
            }
        }
        "WorkTimer".to_owned()
    }

    pub fn theme(&self, _window_id: window::Id) -> Theme {
        self.theme.clone()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.timer.subscription().map(Into::into),
            window::close_events()
                .map(WindowMessage::WindowClosed)
                .map(Into::into),
        ])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Daemon(msg) => self.update_from(msg),
            Message::Load(msg) => self.update_from(msg),
            Message::Save(msg) => self.update_from(msg),
            Message::Window(msg) => self.update_from(msg),
            Message::Settings(msg) => self.settings.update_from(msg),
            Message::Timer(msg) => self.timer.update_from(msg),
        }
    }

    pub fn view(&self, window_id: window::Id) -> Element<Message> {
        if let Some(settings_id) = self.settings_id {
            if settings_id == window_id {
                return self.settings.view().map(Into::into);
            }
        }
        self.timer.view().map(Into::into)
    }

    fn create_config(&self) -> Config {
        let whitelist = self.whitelist.lock().unwrap().clone();
        let on_top = Some(self.timer.get_window_level() == Level::AlwaysOnTop);
        let elapsed = Decimal::from_f32(self.timer.get_elapsed().as_secs_f32()).map(|decimal| {
            decimal.round_dp_with_strategy(3, RoundingStrategy::MidpointAwayFromZero)
        });
        Config {
            elapsed,
            on_top,
            last_pos: None,
            whitelist: Some(whitelist),
        }
    }
}
