mod message;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::assets::fonts::FONT_MONSERRAT;
use crate::state::Config;

use iced::font;
use iced::{window, Element, Subscription, Task, Theme};

use super::settings::Settings;
use super::timer::Timer;

pub use message::*;

#[derive(Debug)]
pub struct Daemon {
    project_file: Option<PathBuf>,
    settings_id: Option<window::Id>,
    config: Arc<Mutex<Config>>,
    timer: Timer,
    settings: Settings,
    theme: Theme,
}

impl Daemon {
    pub fn new(project_file: Option<PathBuf>) -> (Self, Task<Message>) {
        let (config, fallback) = Config::load_or_fallback(project_file.as_ref());
        let config = Arc::new(Mutex::new(config));
        let settings = Settings::new(&config);
        let (timer, task) = Timer::new(&config);
        (
            Self {
                project_file: project_file.or(fallback),
                config,
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

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(settings_id) = self.settings_id {
            if settings_id == window_id {
                return self.settings.view().map(Into::into);
            }
        }
        self.timer.view().map(Into::into)
    }
}
