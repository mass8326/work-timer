use iced::Task;

use crate::app::Daemon;

use super::{Message, SaveMessage, UpdateFrom};

#[derive(Clone, Debug)]
pub enum DaemonMessage {
    Exit,
}

impl UpdateFrom<DaemonMessage, Message> for Daemon {
    fn update_from(&mut self, msg: DaemonMessage) -> Task<Message> {
        match msg {
            DaemonMessage::Exit => {
                let Some(location) = &self.project_file else {
                    return iced::exit();
                };
                self.update_from(SaveMessage::Save(location.clone()))
                    .chain(iced::exit())
                    .discard()
            }
        }
    }
}
