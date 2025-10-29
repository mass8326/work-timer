use std::path::PathBuf;

use iced::Task;
use iced_runtime::window;
use log::error;
use rfd::AsyncFileDialog;

use crate::{app::Daemon, state::Config};

use super::{Message, UpdateFrom};

#[derive(Clone, Debug)]
pub enum SaveMessage {
    SaveCurrent,
    SaveDialog,
    Save(PathBuf),
}

impl UpdateFrom<SaveMessage, Message> for Daemon {
    fn update_from(&mut self, message: SaveMessage) -> Task<Message> {
        match message {
            SaveMessage::SaveCurrent => {
                let Some(path) = self.project_file.as_ref() else {
                    return Task::none();
                };
                Task::done(SaveMessage::Save(path.clone()).into())
            }
            SaveMessage::SaveDialog => {
                let mut dialog = AsyncFileDialog::new()
                    .add_filter("WorkTimer Project", &["wtp"])
                    .set_file_name("config.wtp");
                if let Some(dir) = Config::get_default_project_dir() {
                    dialog = dialog.set_directory(&dir);
                }
                Task::future(async move { dialog.save_file().await })
                    .and_then(|file| Task::done(SaveMessage::Save(file.into()).into()))
            }
            SaveMessage::Save(path) => {
                let snapshot = self.config.lock().unwrap().clone();
                window::get_position(self.timer.get_window_id()).then(move |point| {
                    let config = Config {
                        last_pos: point.map(Into::into),
                        ..snapshot.clone()
                    };
                    if let Err(err) = config.save(&path) {
                        error!("Error occured while saving: {err:#?}");
                    }
                    Task::none()
                })
            }
        }
    }
}
