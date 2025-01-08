use crate::config::VideoSource;
use crate::ui::{ComponentConfig, Message};
use crate::{d_column, d_row, d_sub_title, d_text_input, t};
use iced::widget::{checkbox, text, Column};

pub fn virtual_display<'a>(
    config: &ComponentConfig,
) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = d_sub_title!(
        t! {
            en: "Virtual display",
            zh: "虚拟显示器"
        }
        .to_string(),
    );

    let mut column = d_column![sub_title];

    if config.video_source != VideoSource::Display {
        return column.push(
            text(
                t! {
                    en: "Not using display",
                    zh: "未使用显示器"
                }
                .to_string(),
            )
            .color([0.5, 0.5, 0.5]),
        );
    }

    let enable_virtual_display = checkbox(
        t! {
            en: "Enable virtual display",
            zh: "启用虚拟显示器"
        }
        .to_string(),
        config.virtual_display,
    )
    .on_toggle(Message::VirtualDisplayChanged);

    let display_orientation = if config.display_width >= config.display_height {
        String::from(&t! {
            en: "landscape",
            zh: "横屏"
        })
    } else {
        String::from(&t! {
            en: "portrait",
            zh: "竖屏"
        })
    };

    let display_size = d_row![
        text(
            t! {
                en: "Display size: ",
                zh: "显示器尺寸："
            }
            .to_string(),
        ),
        d_text_input!("", &config.display_width.to_string())
            .width(80)
            .on_input(Message::DisplayWidthChanged),
        text("x"),
        d_text_input!("", &config.display_height.to_string())
            .width(80)
            .on_input(Message::DisplayHeightChanged),
        text(display_orientation.to_string())
    ];

    let destroy_app_on_close = checkbox(
        t! {
            en: "Destroy app on close",
            zh: "关闭时销毁应用"
        }
        .to_string(),
        config.destroy_app_on_close,
    )
    .on_toggle(Message::DestroyAppOnCloseChanged);

    column = column.push(enable_virtual_display);
    if config.virtual_display {
        column = column.push(display_size).push(destroy_app_on_close);
    }
    column
}
