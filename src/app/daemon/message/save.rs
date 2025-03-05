use std::path::PathBuf;

use iced::Task;
use iced_runtime::window;
use rfd::AsyncFileDialog;

use crate::{state::Config, app::Daemon};

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
                let save_data = self.create_config();
                window::get_position(self.timer.get_window_id()).then(move |point| {
                    let config = Config {
                        last_pos: point.map(Into::into),
                        ..save_data.clone()
                    };
                    if let Err(err) = config.save(&path) {
                        println!("Error occured while saving: {err:#?}");
                    };
                    Task::none()
                })
            }
        }
    }
}
