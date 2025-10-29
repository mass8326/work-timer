use std::path::PathBuf;

use iced::Task;

use crate::app::{Message, SaveMessage, Settings, UpdateFrom};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    WhitelistToggle(PathBuf),
}

impl UpdateFrom<SettingsMessage, Message> for Settings {
    fn update_from(&mut self, msg: SettingsMessage) -> Task<Message> {
        match msg {
            SettingsMessage::WhitelistToggle(path) => {
                let mut state = self.config.lock().unwrap();
                state.whitelist.toggle(path);
                Task::done(SaveMessage::SaveCurrent.into())
            }
        }
    }
}
