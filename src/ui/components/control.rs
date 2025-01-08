use crate::ui::{ComponentConfig, Message, StateButton};
use crate::{d_column, d_row, d_sub_title, t};
use iced::widget::{text, Column};

pub fn control<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = d_sub_title!(
        t! {
            en: "Control",
            zh: "控制"
        }
        .to_string(),
    );

    let keyboard = d_row![
        text(
            t! {
                en: "Keyboard input modes: ",
                zh: "键盘输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.keyboard, Message::KeyboardChanged)
    ];
    let mouse = d_row![
        text(
            t! {
                en: "Mouse input modes: ",
                zh: "鼠标输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.mouse, Message::MouseChanged)
    ];
    let gamepad = d_row![
        text(
            t! {
                en: "Gamepad input modes: ",
                zh: "游戏手柄输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.gamepad, Message::GamepadChanged)
    ];

    d_column![sub_title, keyboard, mouse, gamepad]
}
