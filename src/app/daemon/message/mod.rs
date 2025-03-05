mod daemon;
mod load;
mod save;
mod window;

use derive_more::From;
use iced::{self, Task};

pub use super::super::{SettingsMessage, TimerMessage};
pub use daemon::DaemonMessage;
pub use load::LoadMessage;
pub use save::SaveMessage;
pub use window::WindowMessage;

#[derive(Clone, Debug, From)]
pub enum Message {
    Daemon(DaemonMessage),
    Timer(TimerMessage),
    Save(SaveMessage),
    Load(LoadMessage),
    Settings(SettingsMessage),
    Window(WindowMessage),
}

pub trait UpdateFrom<M, R> {
    fn update_from(&mut self, msg: M) -> Task<R>;
}
