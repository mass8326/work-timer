mod clock;
mod message;
mod widget;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use clock::Clock;
use iced::border::Radius;
use iced::time::every;
use iced::widget::container;
use iced::widget::{center, mouse_area};
use iced::window::Level;
use iced::{window, Border, Color, Element, Length, Size, Subscription, Task};
use log::error;
use rust_decimal::prelude::*;
use widget::{clock, danger_controls, safe_controls};

use crate::platform::{ActivityInfo, Platform, PlatformAPI};
use crate::state::Config;

use super::Message;
pub use message::TimerMessage;

#[derive(Clone, Debug, Default)]
pub enum View {
    #[default]
    Clock,
    Controls,
    Danger,
}

#[derive(Debug)]
pub struct Timer {
    activity: ActivityInfo,
    window_level: Level,
    clock: Clock,
    view: View,
    config: Arc<Mutex<Config>>,
    window_id: window::Id,
}

impl Timer {
    pub fn new(config: &Arc<Mutex<Config>>) -> (Self, Task<window::Id>) {
        let state = config.lock().unwrap();
        let level = match state.on_top {
            false => Level::Normal,
            true => Level::AlwaysOnTop,
        };
        let (id, task) = window::open(window::Settings {
            size: Size {
                width: 230_f32,
                height: 60_f32,
            },
            level,
            position: state
                .last_pos
                .clone()
                .map(Into::into)
                .map(window::Position::Specific)
                .unwrap_or_default(),
            resizable: false,
            decorations: false,
            ..Default::default()
        });
        let seconds = state
            .elapsed
            .as_ref()
            .and_then(ToPrimitive::to_f32)
            .map(Duration::from_secs_f32)
            .unwrap_or_default();
        let created = Self {
            activity: ActivityInfo::default(),
            window_level: level,
            clock: seconds.into(),
            view: View::default(),
            window_id: id,
            config: config.clone(),
        };
        let on_top = state.on_top;
        (
            created,
            task.then(move |id| {
                // Window may not be created as always on top
                // Attempt to set it after creation as well just in case
                match on_top {
                    false => Task::none(),
                    true => window::change_level(id, Level::AlwaysOnTop),
                }
            }),
        )
    }

    pub fn subscription(&self) -> Subscription<TimerMessage> {
        let mut subscriptions = Vec::with_capacity(2);
        if self.clock.is_ticking() {
            let ticker = every(Duration::from_millis(100)).map(TimerMessage::Tick);
            subscriptions.push(ticker);
        }
        match Platform::get_activity_stream(Duration::from_millis(500)) {
            Err(err) => error!("{err:?}"),
            Ok(stream) => {
                let sub = Subscription::run_with_id("activity", stream).map(TimerMessage::Activity);
                subscriptions.push(sub);
            }
        }
        Subscription::batch(subscriptions)
    }

    pub fn view(&self) -> Element<'_, Message> {
        mouse_area(
            center(match self.view {
                View::Clock => clock(
                    self.clock.get_elapsed(),
                    &self.config.lock().unwrap().precision.clone(),
                ),
                View::Controls => safe_controls(self.window_level),
                View::Danger => danger_controls(),
            })
            .style(|_| self.get_border_style())
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .on_press(TimerMessage::Move.into())
        .on_right_press(match self.view {
            View::Clock => TimerMessage::SetView(View::Controls).into(),
            View::Controls => TimerMessage::SetView(View::Danger).into(),
            View::Danger => TimerMessage::SetView(View::Clock).into(),
        })
        .into()
    }

    pub fn set_elapsed(&mut self, duration: Duration) {
        self.clock.set_elapsed(duration);
    }

    pub fn get_window_id(&self) -> window::Id {
        self.window_id
    }

    fn get_border_style(&self) -> container::Style {
        container::Style {
            border: Border {
                color: match self.clock.is_ticking() {
                    true => Color {
                        r: 0.3,
                        g: 0.8,
                        b: 0.0,
                        a: 1.0,
                    },
                    false => Color {
                        r: 0.8,
                        g: 0.1,
                        b: 0.0,
                        a: 1.0,
                    },
                },
                width: 5.0,
                radius: Radius::new(0),
            },
            ..Default::default()
        }
    }
}
