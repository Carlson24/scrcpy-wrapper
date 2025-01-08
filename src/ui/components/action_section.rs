use crate::ui::{style_default, Message, WinMain};
use crate::{d_column, d_row, d_text_input, t};
use iced::widget::{button, horizontal_space, text, Column};

pub fn action_section<'a>(win_main: &WinMain) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    d_column![
        d_text_input!(
            &t! {
                en: "Args",
                zh: "参数"
            },
            &win_main.args
        )
        .on_input(Message::ArgsChanged),
        d_row![
            horizontal_space(),
            button(text(t! {en: "Run", zh: "运行"}.to_string())).on_press(Message::Run)
        ]
    ]
    .padding(style_default::Padding::page())
}
