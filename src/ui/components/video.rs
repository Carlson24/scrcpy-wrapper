use crate::config::{OrientationAngle, OrientationType, VideoSource};
use crate::t;
use crate::ui::{style_default, ComponentConfig, Message, StateButton};
use iced::widget::{checkbox, row, text, text_input, Column};
use iced::Center;

pub fn video<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = text(
        t! {
            en: "Video",
            zh: "视频"
        }
        .to_string(),
    )
    .size(style_default::Size::text_sub_title());

    let column = iced::widget::column![sub_title].spacing(style_default::Spacing::general());

    let mut source = row![
        text(
            t! {
                en: "Video source: ",
                zh: "视频源："
            }
            .to_string()
        ),
        StateButton::pick_list(config.video_source.clone(), Message::VideoSourceChanged)
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    if config.video_source == VideoSource::No {
        return column.push(source);
    }

    let camera_size = match &config.video_size {
        None => "".to_string(),
        Some(size) => size.to_string(),
    };
    if config.video_source == VideoSource::Camera {
        source = source.push(StateButton::pick_list(
            config.camera.clone(),
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
            text_input(
                &t! {
                    en: "longest side, e.g. 1920",
                    zh: "最长边，例如 1920"
                },
                &camera_size,
            )
            .on_input(Message::VideoSizeChanged)
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
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

    let codec = row![
        text(
            t! {
                en: "Video codec: ",
                zh: "视频编解码器："
            }
            .to_string()
        ),
        StateButton::pick_list(config.video_codec.clone(), Message::VideoCodecChanged),
        text(
            t! {
                en: "Options: ",
                zh: "参数："
            }
            .to_string()
        ),
        text_input("", &config.video_codec_options,)
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
            .width(300)
            .on_input(Message::VideoCodecOptionsChanged)
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    let mut orientation = row![
        text(
            t! {
                en: "Orientation (clockwise): ",
                zh: "方向 (顺时针)："
            }
            .to_string(),
        ),
        StateButton::button(
            config.orientation_type.clone(),
            Message::OrientationTypeChanged
        ),
        StateButton::pick_list(
            config.orientation_angle.clone(),
            Message::OrientationAngleChanged
        )
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

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
