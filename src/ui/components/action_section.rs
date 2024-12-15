use crate::t;
use crate::ui::{style_default, Message, WinMain};
use iced::widget::{button, horizontal_space, row, text, text_input, Column};
use iced::{Center, Left};

pub fn action_section<'a>(win_main: &WinMain) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    iced::widget::column![
            text_input(
                &t! {
                    en: "Args",
                    zh: "参数"
                },
                &win_main.args,
            )
            .on_input(Message::ArgsChanged)
            .align_x(Center)
            .padding(style_default::Padding::input())
            .size(style_default::Size::input())
            .align_x(Left),
            row![
                horizontal_space(),
                button(text(t! {en: "Run", zh: "运行"}.to_string())).on_press(Message::Run)
            ]
        ]
        .spacing(style_default::Spacing::general())
        .padding(style_default::Padding::page())
}