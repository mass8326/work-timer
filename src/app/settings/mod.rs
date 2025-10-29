mod message;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use iced::widget::{button, column, container, scrollable, text};
use iced::{Alignment, Element, Length};
use iced_core::text::Wrapping;

use crate::platform::{Platform, PlatformAPI, WindowInfo};
use crate::state::Config;

use super::widget::centered_text_button;

pub use message::SettingsMessage;

#[derive(Debug, Default)]
pub struct Settings {
    windows: Vec<WindowInfo>,
    config: Arc<Mutex<Config>>,
}

impl Settings {
    pub fn new(config: &Arc<Mutex<Config>>) -> Self {
        let windows = Platform::get_all_window_info().unwrap_or_default();
        Self {
            windows,
            config: config.clone(),
        }
    }

    pub fn view(&self) -> Element<'_, SettingsMessage> {
        let entries = column(self.windows.iter().filter_map(|info| {
            let path = PathBuf::from(info.path.clone());
            let name = path.file_name()?;
            let label = text!(
                "{} ({})",
                name.to_string_lossy(),
                info.path.to_string_lossy()
            )
            .wrapping(Wrapping::None);
            let guard = self.config.lock().unwrap();
            let style = match guard.whitelist.has(&path) {
                true => button::primary,
                false => button::secondary,
            };
            Some(
                centered_text_button(label.into())
                    .style(style)
                    .width(Length::Fill)
                    .height(34)
                    .on_press(SettingsMessage::WhitelistToggle(path))
                    .into(),
            )
        }))
        .spacing(10);
        column![
            container(text!("Whitelisted Programs").size(32)),
            scrollable(entries)
        ]
        .align_x(Alignment::Center)
        .spacing(10)
        .padding(10)
        .into()
    }
}
