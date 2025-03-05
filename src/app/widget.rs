// use iced::advanced::widget::Text;
use iced::widget::{button, svg, Button};
use iced::{Color, Element, Padding};
use iced_core::svg::Handle;

pub fn centered_text_button<Message>(label: Element<Message>) -> Button<Message> {
    button(label).padding(Padding {
        top: 6.0,
        bottom: 6.0,
        left: 10.0,
        right: 10.0,
    })
}

pub fn icon_button<Message>(icon: &'static [u8]) -> Button<'static, Message> {
    button(
        svg(Handle::from_memory(icon))
            .width(24)
            .height(24)
            .style(|_, _| svg::Style {
                color: Some(Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                }),
            }),
    )
    .padding(Padding::from(4))
}
