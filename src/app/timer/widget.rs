use std::time::Duration;

use iced::widget::{button, column, row, text};
use iced::window::Level;
use iced::{padding, Alignment, Element, Length, Padding};

use crate::app::{DaemonMessage, LoadMessage, Message, SaveMessage, WindowMessage};
use crate::assets::icons::{
    ICON_CLOSE, ICON_FILE_OPEN, ICON_LAYER, ICON_RESET, ICON_SAVE, ICON_SETTINGS,
};
use crate::state::TimerPrecision;

use super::super::widget::icon_button;
use super::TimerMessage;

pub fn clock<'a, Message: 'a>(
    duration: &'a Duration,
    precision: &TimerPrecision,
) -> Element<'a, Message> {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    let seconds = duration.as_secs();
    let inner = match precision {
        TimerPrecision::Decisecond => text!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>1}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            duration.subsec_millis() / 100
        ),
        TimerPrecision::Second => text!(
            "{:0>2}:{:0>2}:{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
        ),
    }
    .size(40)
    .center();
    column![inner]
        .align_x(Alignment::Center)
        .padding(
            #[cfg(target_os = "windows")]
            padding::bottom(6),
            #[cfg(not(target_os = "windows"))]
            padding::bottom(2),
        )
        .into()
}

pub fn safe_controls<'a>(level: Level) -> Element<'a, Message> {
    controls(
        row![
            icon_button(ICON_SAVE).on_press(SaveMessage::SaveDialog.into()),
            icon_button(ICON_FILE_OPEN).on_press(LoadMessage::LoadDialog.into()),
            icon_button(ICON_LAYER)
                .style(match level {
                    Level::AlwaysOnTop => button::primary,
                    _ => button::secondary,
                })
                .on_press(TimerMessage::ToggleLevel.into()),
            icon_button(ICON_SETTINGS).on_press(WindowMessage::OpenSettingsFindPosition.into()),
        ]
        .spacing(10)
        .into(),
    )
}

pub fn danger_controls<'a>() -> Element<'a, Message> {
    controls(
        row![icon_button(ICON_RESET)
            .style(button::danger)
            .on_press(TimerMessage::Reset.into()),]
        .spacing(10)
        .into(),
    )
}

fn controls(content: Element<Message>) -> Element<Message> {
    let close_button = icon_button(ICON_CLOSE)
        .style(button::danger)
        .on_press(DaemonMessage::Exit.into());
    row![
        close_button,
        column![content].width(Length::Fill).align_x(Alignment::End)
    ]
    .padding(Padding::from([0, 15]))
    .into()
}
