use iced::Center;
use iced::widget::{row, text, Row};
use crate::t;
use crate::ui::{style_default, ComponentConfig, Message, StateButton};

pub fn connect_method<'a>(config: &ComponentConfig) -> Row<'a, Message, iced::Theme, iced::Renderer> {
    row![
            text(&t! {r
                zh: "连接方式：",
                en: "Connecting method: "
            }),
            StateButton::button(config.connect_method.clone(), Message::ConnectMethodChanged)
        ]
        .spacing(style_default::Spacing::general())
        .align_y(Center)
}