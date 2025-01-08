use crate::config::{OrientationAngle, OrientationType, VideoSource};
use crate::ui::{ComponentConfig, Message, StateButton};
use crate::{d_column, d_row, d_sub_title, d_text_input, t};
use iced::widget::{checkbox, text, Column};

pub fn video<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = d_sub_title!(t! {
        en: "Video",
        zh: "视频"
    }
    .to_string());

    let column = d_column![sub_title];

    let mut source = d_row![
        text(
            t! {
                en: "Video source: ",
                zh: "视频源："
            }
            .to_string()
        ),
        StateButton::pick_list(config.video_source, Message::VideoSourceChanged)
    ];

    if config.video_source == VideoSource::No {
        return column.push(source);
    }

    let camera_size = match &config.video_size {
        None => "".to_string(),
        Some(size) => size.to_string(),
    };
    if config.video_source == VideoSource::Camera {
        source = source.push(StateButton::pick_list(
            config.camera,
            Message::CameraChanged,
        ))
    }

    source = source
        .push(text(
            t! {
                en: "Size: ",
                zh: "分辨率："
            }
            .to_string(),
        ))
        .push(
            d_text_input!(
                &t! {
                    en: "longest side, e.g. 1920",
                    zh: "最长边，例如 1920"
                },
                &camera_size,
            )
            .on_input(Message::VideoSizeChanged)
            .width(180),
        )
        .push(
            checkbox(
                t! {
                    en: "playback",
                    zh: "播放"
                }
                .to_string(),
                config.video_playback,
            )
            .on_toggle(Message::VideoPlaybackChanged),
        );

    let codec = d_row![
        text(
            t! {
                en: "Video codec: ",
                zh: "视频编解码器："
            }
            .to_string()
        ),
        StateButton::pick_list(config.video_codec, Message::VideoCodecChanged),
        text(
            t! {
                en: "Options: ",
                zh: "参数："
            }
            .to_string()
        ),
        d_text_input!("", &config.video_codec_options)
            .width(300)
            .on_input(Message::VideoCodecOptionsChanged)
    ];

    let mut orientation = d_row![
        text(
            t! {
                en: "Orientation (clockwise): ",
                zh: "方向 (顺时针)："
            }
            .to_string(),
        ),
        StateButton::button(config.orientation_type, Message::OrientationTypeChanged),
        StateButton::pick_list(config.orientation_angle, Message::OrientationAngleChanged)
    ];

    if config.orientation_type == OrientationType::Capture {
        orientation = orientation.push(
            checkbox(
                t! {
                    en: "Lock orientation",
                    zh: "锁定方向"
                }
                .to_string(),
                config.orientation_lock,
            )
            .on_toggle(Message::OrientationLockChanged),
        );
    }

    if config.orientation_angle != OrientationAngle::Default {
        orientation = orientation.push(
            checkbox(
                t! {
                    en: "Flip",
                    zh: "翻转"
                }
                .to_string(),
                config.orientation_flip,
            )
            .on_toggle(Message::OrientationFlipChanged),
        );
    }

    column.push(source).push(codec).push(orientation)
}
