use crate::t;
use crate::ui::{style_default, ComponentConfig, Message, StateButton};
use iced::widget::{column, row, text, Column};

pub fn control<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = text(
        t! {
            en: "Control",
            zh: "控制"
        }
        .to_string(),
    )
    .size(style_default::Size::text_sub_title());

    let keyboard = row![
        text(
            t! {
                en: "Keyboard input modes: ",
                zh: "键盘输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.keyboard.clone(), Message::KeyboardChanged)
    ];
    let mouse = row![
        text(
            t! {
                en: "Mouse input modes: ",
                zh: "鼠标输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.mouse.clone(), Message::MouseChanged)
    ];
    let gamepad = row![
        text(
            t! {
                en: "Gamepad input modes: ",
                zh: "游戏手柄输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.gamepad.clone(), Message::GamepadChanged)
    ];

    column![sub_title, keyboard, mouse, gamepad]
        .spacing(style_default::Spacing::general())
}
