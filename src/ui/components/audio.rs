use crate::config::AudioSource;
use crate::ui::{ComponentConfig, Message, StateButton};
use crate::{d_column, d_row, d_sub_title, d_text_input, t};
use iced::widget::{checkbox, text, Column};

pub fn audio<'a>(config: &ComponentConfig) -> Column<'a, Message, iced::Theme, iced::Renderer> {
    let sub_title = d_sub_title!(
        t! {
            en: "Audio",
            zh: "音频"
        }
        .to_string(),
    );

    let column = d_column![sub_title];

    let mut source = d_row![
        text(
            t! {
                en: "Audio source: ",
                zh: "音频源："
            }
            .to_string()
        ),
        StateButton::pick_list(config.audio_source, Message::AudioSourceChanged)
    ];

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

    let codec = d_row![
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
        d_text_input!("", &config.audio_codec_options)
            .width(300)
            .on_input(Message::AudioCodecOptionsChanged)
    ];

    column.push(source).push(codec)
}
