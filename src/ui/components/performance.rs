use crate::config::{AudioCodec, AudioSource, VideoSource};
use crate::t;
use crate::ui::{style_default, ComponentConfig, Message};
use iced::widget::{checkbox, row, text, text_input, Column};
use iced::Center;

pub fn performance<'a>(
    config: &ComponentConfig,
) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let have_audio = config.audio_source != AudioSource::No;
    let have_video = config.video_source != VideoSource::No;

    let sub_title = text(
        t! {
            en: "Performance",
            zh: "性能"
        }
        .to_string(),
    )
    .size(style_default::Size::text_sub_title());

    let mut column = iced::widget::column![sub_title].spacing(style_default::Spacing::general());

    if !have_audio && !have_video {
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

    let mut bit_rate = row![text(
        t! {
            en: "Bit rate: ",
            zh: "码率："
        }
        .to_string()
    )]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    if have_video {
        bit_rate = bit_rate
            .push(text(
                t! {
                    en: "video",
                    zh: "视频"
                }
                .to_string(),
            ))
            .push(
                text_input("8M", &config.video_bit_rate.to_string())
                    .size(style_default::Size::input())
                    .padding(style_default::Padding::input())
                    .on_input(Message::VideoBitRateChanged)
                    .width(100),
            );
    }

    if have_audio && config.audio_codec != AudioCodec::Raw {
        bit_rate = bit_rate
            .push(text(
                t! {
                    en: "audio",
                    zh: "音频"
                }
                .to_string(),
            ))
            .push(
                text_input("128K", &config.audio_bit_rate.to_string())
                    .size(style_default::Size::input())
                    .padding(style_default::Padding::input())
                    .on_input(Message::AudioBitRateChanged)
                    .width(100),
            );
    }

    if have_video || (have_audio && config.audio_codec != AudioCodec::Raw) {
        column = column.push(bit_rate);
    }

    if have_video {
        let fps = row![
            if config.video_source == VideoSource::Camera {
                text(
                    t! {
                        en: "FPS: ",
                        zh: "帧率："
                    }
                    .to_string(),
                )
            } else {
                text(
                    t! {
                        en: "Max FPS: ",
                        zh: "最大帧率："
                    }
                    .to_string(),
                )
            },
            text_input(
                "",
                &match &config.fps {
                    None => "".to_string(),
                    Some(fps) => fps.to_string(),
                }
            )
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
            .on_input(Message::FpsChanged)
            .width(60)
        ]
        .spacing(style_default::Spacing::general())
        .align_y(Center);
        column = column.push(fps);
    }

    let mut buffer = row![text(
        t! {
            en: "Buffer: ",
            zh: "缓冲："
        }
        .to_string(),
    )]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    if have_video {
        buffer = buffer
            .push(text(
                t! {
                    en: "video",
                    zh: "视频"
                }
                .to_string(),
            ))
            .push(
                text_input(
                    "50",
                    &match &config.video_buffer {
                        None => "".to_string(),
                        Some(buffer) => buffer.to_string(),
                    },
                )
                .size(style_default::Size::input())
                .padding(style_default::Padding::input())
                .on_input(Message::VideoBufferChanged)
                .width(52),
            )
            .push(text("ms"));
    }
    if have_audio {
        buffer = buffer
            .push(text(
                t! {
                    en: "audio",
                    zh: "音频"
                }
                .to_string(),
            ))
            .push(
                text_input(
                    "50",
                    &match &config.audio_buffer {
                        None => "".to_string(),
                        Some(buffer) => buffer.to_string(),
                    },
                )
                .size(style_default::Size::input())
                .padding(style_default::Padding::input())
                .on_input(Message::AudioBufferChanged)
                .width(52),
            )
            .push(text("ms"));
    }
    if have_video && have_audio {
        buffer = buffer.push(
            checkbox(
                t! {
                    en: "Sync",
                    zh: "同步"
                }
                .to_string(),
                config.buffer_sync,
            )
            .on_toggle(Message::BufferSyncChanged),
        );
    }

    column.push(buffer)
}
