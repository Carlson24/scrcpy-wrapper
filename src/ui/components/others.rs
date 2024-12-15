use crate::t;
use crate::ui::{style_default, ComponentConfig, Message, StateButton};
use iced::widget::{checkbox, column, row, text, text_input, Column};
use iced::Center;

// pub start_app: String,
// pub restart_app: bool,
// pub app_name_type: AppNameType,

pub fn others<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = text(
        t! {
            en: "Others",
            zh: "其他"
        }
        .to_string(),
    )
    .size(style_default::Size::text_sub_title());

    let start_app = row![
        text(
            t! {
                en: "Start app: ",
                zh: "启动应用："
            }
            .to_string()
        )
        .size(style_default::Size::text_in_button()),
        StateButton::button(config.app_name_type.clone(), Message::AppNameTypeChanged),
        text_input("", &config.start_app)
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
            .width(200)
            .on_input(Message::StartAppChanged),
        checkbox(
            t! {en: "Restart if running", zh: "如果正在运行则重启"}.to_string(),
            config.restart_app,
        )
        .on_toggle(Message::RestartAppChanged),
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    let time_limit = row![
        text(
            t! {
                en: "Time limit: ",
                zh: "时间限制："
            }
            .to_string()
        )
        .size(style_default::Size::text_in_button()),
        text_input(
            "",
            &match &config.time_limit {
                Some(time_limit) => time_limit.to_string(),
                None => "".to_string(),
            },
        )
        .size(style_default::Size::input())
        .padding(style_default::Padding::input())
        .width(100)
        .on_input(Message::TimeLimitChanged),
        text("s")
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    let stay_awake = checkbox(
        t! {en: "Stay awake (device)", zh: "保持唤醒（设备）"}.to_string(),
        config.stay_awake,
    )
    .on_toggle(Message::StayAwakeChanged);

    let disable_window = checkbox(
        t! {en: "Disable window", zh: "禁用窗口"}.to_string(),
        config.disable_window,
    )
    .on_toggle(Message::DisableWindowChanged);

    let borderless = checkbox(
        t! {en: "Borderless", zh: "无边框"}.to_string(),
        config.borderless,
    )
    .on_toggle(Message::BorderlessChanged);

    let always_on_top = checkbox(
        t! {en: "Always on top", zh: "窗口置顶"}.to_string(),
        config.always_on_top,
    )
    .on_toggle(Message::AlwaysOnTopChanged);

    let fullscreen = checkbox(
        t! {en: "Fullscreen", zh: "全屏"}.to_string(),
        config.fullscreen,
    )
    .on_toggle(Message::FullscreenChanged);

    let disable_screensaver = checkbox(
        t! {en: "Disable screensaver (PC)", zh: "禁用屏保（电脑）"}.to_string(),
        config.disable_screensaver,
    )
    .on_toggle(Message::DisableScreensaverChanged);

    let additional_args = row![
        text(
            t! {
                en: "Additional arguments",
                zh: "附加参数"
            }
            .to_string()
        )
        .size(style_default::Size::text_in_button()),
        text_input("", &config.additional_args)
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
            .width(700)
            .on_input(Message::AdditionalArgsChanged)
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    column![
        sub_title,
        start_app,
        time_limit,
        row![stay_awake]
            .spacing(style_default::Spacing::general())
            .align_y(Center),
        row![disable_window, borderless]
            .spacing(style_default::Spacing::general())
            .align_y(Center),
        row![always_on_top, fullscreen]
            .spacing(style_default::Spacing::general())
            .align_y(Center),
        disable_screensaver,
        additional_args
    ]
    .spacing(style_default::Spacing::general())
}
