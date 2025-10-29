use std::{path::PathBuf, time::Instant};

use iced::{window::Level, Task};
use iced_runtime::window;

use crate::{
    app::{
        timer::{Timer, View},
        Message, UpdateFrom,
    },
    platform::ActivityInfo,
};

const IDLE_TIME_LIMIT_MS: u32 = 7500;

#[derive(Clone, Debug)]
pub enum TimerMessage {
    Reset,
    Move,
    ToggleLevel,
    SetView(View),
    Tick(Instant),
    Activity(ActivityInfo),
}

impl UpdateFrom<TimerMessage, Message> for Timer {
    fn update_from(&mut self, msg: TimerMessage) -> Task<Message> {
        match msg {
            TimerMessage::ToggleLevel => {
                self.window_level = match self.window_level {
                    Level::Normal => Level::AlwaysOnTop,
                    _ => Level::Normal,
                };
                return window::change_level(self.window_id, self.window_level);
            }
            TimerMessage::Reset => {
                self.clock.reset();
            }
            TimerMessage::Move => {
                return window::get_oldest().then(|opt| opt.map_or_else(Task::none, window::drag))
            }
            TimerMessage::SetView(view) => {
                self.view = view;
            }
            TimerMessage::Tick(now) => {
                self.clock.tick(now);
            }
            TimerMessage::Activity(state) => {
                if let Some(info) = &state.window {
                    if info.pid != std::process::id() {
                        self.view = View::Clock;
                    }
                } else {
                    self.view = View::Clock;
                }
                self.activity = state.clone();
                let Some(window) = &state.window else {
                    self.clock.off();
                    return Task::none();
                };
                if !self
                    .config
                    .lock()
                    .unwrap()
                    .whitelist
                    .has(&PathBuf::from(&window.path))
                {
                    self.clock.off();
                    return Task::none();
                }
                if state.idle_time > IDLE_TIME_LIMIT_MS {
                    self.clock.off();
                    return Task::none();
                }
                self.clock.on();
            }
        }
        Task::none()
    }
}
