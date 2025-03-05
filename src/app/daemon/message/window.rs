use iced::{Point, Size, Task};

use crate::app::Daemon;

use super::{Message, UpdateFrom};

#[derive(Clone, Debug)]
pub enum WindowMessage {
    OpenSettingsFindPosition,
    OpenSettings(Option<Point>),
    WindowClosed(iced::window::Id),
}

impl UpdateFrom<WindowMessage, Message> for Daemon {
    fn update_from(&mut self, msg: WindowMessage) -> Task<Message> {
        match msg {
            WindowMessage::OpenSettingsFindPosition => {
                if let Some(settings_id) = self.settings_id {
                    return iced::window::gain_focus(settings_id);
                }
                iced::window::get_position(self.timer.get_window_id())
                    .map(|point| WindowMessage::OpenSettings(point).into())
            }
            WindowMessage::OpenSettings(point) => {
                if let Some(settings_id) = self.settings_id {
                    return iced::window::gain_focus(settings_id);
                }
                let size = Size {
                    width: 420.0,
                    height: 420.0,
                };
                let position = match point {
                    Some(point) => iced::window::Position::Specific(Point {
                        x: point.x,
                        y: point.y + 80.0,
                    }),
                    None => iced::window::Position::Centered,
                };
                let (id, task) = iced::window::open(iced::window::Settings {
                    size,
                    position,
                    ..Default::default()
                });
                self.settings_id = Some(id);
                task.discard()
            }
            WindowMessage::WindowClosed(id) => {
                if let Some(settings_id) = self.settings_id {
                    if settings_id == id {
                        self.settings_id = None;
                    }
                }
                Task::none()
            }
        }
    }
}
