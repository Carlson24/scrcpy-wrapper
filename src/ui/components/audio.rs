use crate::config::AudioSource;
use crate::t;
use crate::ui::{style_default, ComponentConfig, Message, StateButton};
use iced::widget::{checkbox, column, row, text, text_input, Column};
use iced::Center;

pub fn audio<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = text(
        t! {
            en: "Audio",
            zh: "音频"
        }
        .to_string(),
    )
    .size(style_default::Size::text_sub_title());

    let column = column![sub_title].spacing(style_default::Spacing::general());

    let mut source = row![
        text(
            t! {
                en: "Audio source: ",
                zh: "音频源："
            }
            .to_string()
        ),
        StateButton::pick_list(config.audio_source, Message::AudioSourceChanged)
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    if config.audio_source == AudioSource::No {
        return column.push(source);
    }

    if config.audio_source == AudioSource::Playback {
        source = source.push(
            checkbox(
                t! {
                    en: "duplicate audio",
                    zh: "复制音频"
                }
                .to_string(),
                config.audio_dup,
            )
            .on_toggle(Message::AudioDupChanged),
        );
    }

    source = source.push(
        checkbox(
            t! {
                en: "playback",
                zh: "播放"
            }
            .to_string(),
            config.audio_playback,
        )
        .on_toggle(Message::AudioPlaybackChanged),
    );

    let codec = row![
        text(
            t! {
                en: "Audio codec: ",
                zh: "音频编解码器："
            }
            .to_string()
        ),
        StateButton::pick_list(config.audio_codec, Message::AudioCodecChanged),
        text(
            t! {
                en: "Options: ",
                zh: "参数："
            }
            .to_string()
        ),
        text_input("", &config.audio_codec_options,)
            .size(style_default::Size::input())
            .padding(style_default::Padding::input())
            .width(300)
            .on_input(Message::AudioCodecOptionsChanged)
    ]
    .spacing(style_default::Spacing::general())
    .align_y(Center);

    column.push(source).push(codec)
}
