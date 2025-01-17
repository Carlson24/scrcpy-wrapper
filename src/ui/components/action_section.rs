use crate::ui::{style_default, Message};
use crate::{d_column, d_row, d_text_input, define_component, t};
use iced::widget::{button, horizontal_space, text};

define_component!(action_section, |_, win_main| {
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
    .into()
});
