use std::{path::PathBuf, time::Duration};

use iced::Task;
use rfd::AsyncFileDialog;
use rust_decimal::{prelude::ToPrimitive, Decimal};

use crate::{
    app::{
        timer::{TimerMessage, View},
        Daemon,
    },
    state::Config,
};

use super::{Message, UpdateFrom};

#[derive(Clone, Debug)]
pub enum LoadMessage {
    LoadDialog,
    Load(PathBuf),
}

impl UpdateFrom<LoadMessage, Message> for Daemon {
    fn update_from(&mut self, msg: LoadMessage) -> Task<Message> {
        match msg {
            LoadMessage::LoadDialog => {
                let mut dialog = AsyncFileDialog::new().add_filter("WorkTimer Project", &["wtp"]);
                if let Some(dir) = Config::get_default_project_dir() {
                    dialog = dialog.set_directory(dir);
                }
                Task::future(async move { dialog.pick_file().await })
                    .and_then(|file| Task::done(LoadMessage::Load(file.into()).into()))
            }
            LoadMessage::Load(path) => {
                let Ok(loaded) = Config::load(&path) else {
                    return Task::none();
                };
                self.project_file = Some(path);
                let elapsed = loaded
                    .elapsed
                    .as_ref()
                    .and_then(Decimal::to_f32)
                    .map(Duration::from_secs_f32)
                    .unwrap_or_default();
                self.timer.set_elapsed(elapsed);
                let mut state = self.config.lock().unwrap();
                *state = loaded;
                Task::done(TimerMessage::SetView(View::Clock).into())
            }
        }
    }
}
