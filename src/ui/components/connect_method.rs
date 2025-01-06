use crate::t;
use crate::ui::{style_default, ComponentConfig, Message, StateButton};
use iced::widget::{horizontal_space, row, text, Row};
use iced::Center;

pub fn connect_method<'a>(
    config: &ComponentConfig,
) -> Row<'a, Message, iced::Theme, iced::Renderer> {
    row![
        text(&t! {r
            zh: "连接方式：",
            en: "Connecting method: "
        }),
        StateButton::button(config.connect_method, Message::ConnectMethodChanged),
        horizontal_space(),
        StateButton::button(config.language, Message::LanguageChanged)
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center)
}
