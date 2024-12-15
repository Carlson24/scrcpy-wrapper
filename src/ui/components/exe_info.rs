use crate::t;
use crate::ui::{style_default, ComponentConfig, Message};
use iced::widget::{horizontal_space, row, text, text_input, Row};
use iced::{Center, Left};

pub fn exe_info<'a>(config: &ComponentConfig) -> Row<'a, Message, iced::Theme, iced::Renderer> {
    let executable_path = {
        let exe_str = match &config.executable {
            None => String::from(""),
            Some(p) => p.clone(),
        };
        text_input(
            &t! {
                en: "Executable path",
                zh: "可执行文件路径"
            },
            &exe_str,
        )
        .on_input(Message::ExecutablePathChanged)
        .align_x(Center)
        .padding(style_default::Padding::input())
        .size(style_default::Size::input())
        .width(500)
        .align_x(Left)
    };
    let executable_path_row = row![
        text(&t! {r
            en: "scrcpy executable path: ",
            zh: "scrcpy 可执行文件路径："
        }),
        executable_path,
        horizontal_space()
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    executable_path_row
}
