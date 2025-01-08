use crate::config::{AudioSource, VideoSource};
use crate::ui::{ComponentConfig, Message};
use crate::{d_column, d_row, d_sub_title, d_text_input, t};
use iced::widget::{text, Column};

pub fn output<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = d_sub_title!(
        t! {
            en: "Output",
            zh: "输出"
        }
        .to_string(),
    );

    let column = d_column![sub_title];

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

    let record = d_row![
        text(
            t! {
                en: "Record file: ",
                zh: "录制文件："
            }
            .to_string(),
        ),
        d_text_input!("", &config.record)
            .width(500)
            .on_input(Message::RecordChanged),
    ];

    let v4l2 = d_row![
        text(
            t! {
                en: "V4L2 device: ",
                zh: "V4L2 设备："
            }
            .to_string(),
        ),
        d_text_input!("", &config.v4l2)
            .width(500)
            .on_input(Message::V4l2Changed),
    ];

    column.push(record).push(v4l2)
}
