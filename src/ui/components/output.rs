use crate::config::{AudioSource, VideoSource};
use crate::t;
use crate::ui::{style_default, ComponentConfig, Message};
use iced::widget::{column, row, text, text_input, Column};
use iced::Center;

pub fn output<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = text(
        t! {
            en: "Output",
            zh: "输出"
        }
        .to_string(),
    )
    .size(style_default::Size::text_sub_title());

    let column = column![sub_title].spacing(style_default::Spacing::general());

    if config.video_source == VideoSource::No && config.audio_source == AudioSource::No {
        return column.push(
            text(
                t! {
                    en: "No audio and video source",
                    zh: "没有音频和视频源"
                }
                .to_string(),
            )
            .color([0.5, 0.5, 0.5]),
        );
    }

    let record = row![
        text(
            t! {
                en: "Record file: ",
                zh: "录制文件："
            }
            .to_string(),
        ),
        text_input("", &config.record)
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
            .width(500)
            .on_input(Message::RecordChanged),
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    let v4l2 = row![
        text(
            t! {
                en: "V4L2 device: ",
                zh: "V4L2 设备："
            }
            .to_string(),
        ),
        text_input("", &config.v4l2)
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
            .width(500)
            .on_input(Message::V4l2Changed),
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    column.push(record).push(v4l2)
}
