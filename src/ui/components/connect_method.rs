use crate::ui::{ComponentConfig, Message, StateButton};
use crate::{d_row, t};
use iced::widget::{horizontal_space, text, Row};

pub fn connect_method<'a>(
    config: &ComponentConfig,
) -> Row<'a, Message, iced::Theme, iced::Renderer> {
    d_row![
        text(&t! {r
            zh: "连接方式：",
            en: "Connecting method: "
        }),
        StateButton::button(config.connect_method, Message::ConnectMethodChanged),
        horizontal_space(),
        StateButton::button(config.language, Message::LanguageChanged)
    ]
}
