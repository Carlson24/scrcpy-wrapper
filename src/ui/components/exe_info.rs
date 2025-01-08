use crate::ui::{ComponentConfig, Message};
use crate::{d_row, d_text_input, t};
use iced::widget::{text, Row};

pub fn exe_info<'a>(config: &ComponentConfig) -> Row<'a, Message, iced::Theme, iced::Renderer> {
    let executable_path = {
        let exe_str = match &config.executable {
            None => String::from(""),
            Some(p) => p.clone(),
        };
        d_text_input!(
            &t! {
                en: "Executable path",
                zh: "可执行文件路径"
            },
            &exe_str,
        )
        .on_input(Message::ExecutablePathChanged)
    };
    let executable_path_row = d_row![
        text(&t! {r
            en: "scrcpy executable path: ",
            zh: "scrcpy 可执行文件路径："
        }),
        executable_path,
    ];

    executable_path_row
}
