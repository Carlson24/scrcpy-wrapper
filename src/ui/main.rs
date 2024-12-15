use crate::config::{
    AppNameType, AudioCodec, AudioSource, Camera, Config, ConnectMethod, Gamepad, Keyboard, Mouse,
    OrientationAngle, OrientationType, VideoCodec, VideoSource,
};
use crate::ui::{components, style_default};
use crate::util::build_args;
use crate::{ARGS, CONFIG};
use dark_light::Mode;
use iced::widget::container::Id;
use iced::widget::{column, container, horizontal_rule, scrollable};
use iced::window::close;
use iced::{window, Element, Size, Subscription, Task, Theme};
use std::sync::RwLockReadGuard;

pub struct WinMain {
    pub(crate) args: String,
    size: Size,
}

impl Default for WinMain {
    fn default() -> Self {
        Self {
            args: String::new(),
            size: Size {
                width: 800.0,
                height: 600.0,
            },
        }
    }
}

pub type ComponentConfig = RwLockReadGuard<'static, Box<Config>>;

#[derive(Debug, Clone)]
pub enum Message {
    ExecutablePathChanged(String),
    ConnectMethodChanged(ConnectMethod),
    VideoSourceChanged(VideoSource),
    CameraChanged(Camera),
    VideoSizeChanged(String),
    VideoPlaybackChanged(bool),
    VideoCodecChanged(VideoCodec),
    VideoCodecOptionsChanged(String),
    OrientationTypeChanged(OrientationType),
    OrientationAngleChanged(OrientationAngle),
    OrientationLockChanged(bool),
    OrientationFlipChanged(bool),
    AudioSourceChanged(AudioSource),
    AudioDupChanged(bool),
    AudioPlaybackChanged(bool),
    AudioCodecChanged(AudioCodec),
    AudioCodecOptionsChanged(String),
    VideoBitRateChanged(String),
    AudioBitRateChanged(String),
    FpsChanged(String),
    VideoBufferChanged(String),
    AudioBufferChanged(String),
    BufferSyncChanged(bool),
    KeyboardChanged(Keyboard),
    MouseChanged(Mouse),
    GamepadChanged(Gamepad),
    RecordChanged(String),
    V4l2Changed(String),
    VirtualDisplayChanged(bool),
    DisplayHeightChanged(String),
    DisplayWidthChanged(String),
    DestroyAppOnCloseChanged(bool),
    StartAppChanged(String),
    RestartAppChanged(bool),
    AppNameTypeChanged(AppNameType),
    TimeLimitChanged(String),
    StayAwakeChanged(bool),
    DisableWindowChanged(bool),
    BorderlessChanged(bool),
    AlwaysOnTopChanged(bool),
    FullscreenChanged(bool),
    DisableScreensaverChanged(bool),
    AdditionalArgsChanged(String),

    ArgsChanged(String),
    Run,
    Resize(Size),
}
impl WinMain {
    pub fn update(&mut self, message: Message) -> impl Into<Task<Message>> {
        match message {
            Message::ExecutablePathChanged(path) => {
                CONFIG.write().unwrap().executable = Some(path);
                self.args = build_args();
            }
            Message::ConnectMethodChanged(method) => {
                CONFIG.write().unwrap().connect_method = method;
                self.args = build_args();
            }
            Message::VideoSourceChanged(source) => {
                CONFIG.write().unwrap().video_source = source;
                self.args = build_args();
            }
            Message::CameraChanged(camera) => {
                CONFIG.write().unwrap().camera = camera;
                self.args = build_args();
            }
            Message::VideoSizeChanged(size) => {
                if size.trim().is_empty() {
                    CONFIG.write().unwrap().video_size = None
                } else if let Ok(size) = size.parse::<u32>() {
                    CONFIG.write().unwrap().video_size = Some(size)
                };
                self.args = build_args();
            }
            Message::VideoPlaybackChanged(display) => {
                CONFIG.write().unwrap().video_playback = display;
                self.args = build_args();
            }
            Message::VideoCodecChanged(codec) => {
                CONFIG.write().unwrap().video_codec = codec;
                self.args = build_args();
            }
            Message::VideoCodecOptionsChanged(options) => {
                CONFIG.write().unwrap().video_codec_options = options;
                self.args = build_args();
            }
            Message::OrientationTypeChanged(orientation) => {
                CONFIG.write().unwrap().orientation_type = orientation;
                self.args = build_args();
            }
            Message::OrientationAngleChanged(angle) => {
                CONFIG.write().unwrap().orientation_angle = angle;
                self.args = build_args();
            }
            Message::OrientationLockChanged(lock) => {
                CONFIG.write().unwrap().orientation_lock = lock;
                self.args = build_args();
            }
            Message::OrientationFlipChanged(flip) => {
                CONFIG.write().unwrap().orientation_flip = flip;
                self.args = build_args();
            }
            Message::AudioSourceChanged(source) => {
                CONFIG.write().unwrap().audio_source = source;
                self.args = build_args();
            }
            Message::AudioDupChanged(dup) => {
                CONFIG.write().unwrap().audio_dup = dup;
                self.args = build_args();
            }
            Message::AudioPlaybackChanged(play) => {
                CONFIG.write().unwrap().audio_playback = play;
                self.args = build_args();
            }
            Message::AudioCodecChanged(codec) => {
                CONFIG.write().unwrap().audio_codec = codec;
                self.args = build_args();
            }
            Message::AudioCodecOptionsChanged(options) => {
                CONFIG.write().unwrap().audio_codec_options = options;
                self.args = build_args();
            }
            Message::VideoBitRateChanged(rate) => {
                CONFIG.write().unwrap().video_bit_rate = rate.trim().to_string();
                self.args = build_args();
            }
            Message::AudioBitRateChanged(rate) => {
                CONFIG.write().unwrap().audio_bit_rate = rate.trim().to_string();
                self.args = build_args();
            }
            Message::FpsChanged(fps) => {
                if fps.trim().is_empty() {
                    CONFIG.write().unwrap().fps = None
                } else if let Ok(size) = fps.parse::<u32>() {
                    CONFIG.write().unwrap().fps = Some(size)
                };
                self.args = build_args();
            }
            Message::VideoBufferChanged(buffer) => {
                if buffer.trim().is_empty() {
                    CONFIG.write().unwrap().video_buffer = None;
                    if CONFIG.read().unwrap().buffer_sync {
                        CONFIG.write().unwrap().audio_buffer = None;
                    }
                } else if let Ok(size) = buffer.parse::<u32>() {
                    CONFIG.write().unwrap().video_buffer = Some(size);
                    if CONFIG.read().unwrap().buffer_sync {
                        CONFIG.write().unwrap().audio_buffer = Some(size);
                    }
                };
                self.args = build_args();
            }
            Message::AudioBufferChanged(buffer) => {
                if buffer.trim().is_empty() {
                    CONFIG.write().unwrap().audio_buffer = None;
                    if CONFIG.read().unwrap().buffer_sync {
                        CONFIG.write().unwrap().video_buffer = None;
                    }
                } else if let Ok(size) = buffer.parse::<u32>() {
                    CONFIG.write().unwrap().audio_buffer = Some(size);
                    if CONFIG.read().unwrap().buffer_sync {
                        CONFIG.write().unwrap().video_buffer = Some(size);
                    }
                };
                self.args = build_args();
            }
            Message::BufferSyncChanged(sync) => {
                CONFIG.write().unwrap().buffer_sync = sync;
                self.args = build_args();
            }
            Message::KeyboardChanged(keyboard) => {
                CONFIG.write().unwrap().keyboard = keyboard;
                self.args = build_args();
            }
            Message::MouseChanged(mouse) => {
                CONFIG.write().unwrap().mouse = mouse;
                self.args = build_args();
            }
            Message::GamepadChanged(gamepad) => {
                CONFIG.write().unwrap().gamepad = gamepad;
                self.args = build_args();
            }
            Message::RecordChanged(record) => {
                CONFIG.write().unwrap().record = record;
                self.args = build_args();
            }
            Message::V4l2Changed(v4l2) => {
                CONFIG.write().unwrap().v4l2 = v4l2;
                self.args = build_args();
            }
            Message::VirtualDisplayChanged(virtual_display) => {
                CONFIG.write().unwrap().virtual_display = virtual_display;
                self.args = build_args();
            }
            Message::DisplayHeightChanged(height) => {
                if height.trim().is_empty() {
                    CONFIG.write().unwrap().display_height = 0
                } else if let Ok(size) = height.parse::<u32>() {
                    CONFIG.write().unwrap().display_height = size
                };
                self.args = build_args();
            }
            Message::DisplayWidthChanged(width) => {
                if width.trim().is_empty() {
                    CONFIG.write().unwrap().display_width = 0
                } else if let Ok(size) = width.parse::<u32>() {
                    CONFIG.write().unwrap().display_width = size
                };
                self.args = build_args();
            }
            Message::DestroyAppOnCloseChanged(destroy_app_on_close) => {
                CONFIG.write().unwrap().destroy_app_on_close = destroy_app_on_close;
                self.args = build_args();
            }
            Message::StartAppChanged(start_app) => {
                CONFIG.write().unwrap().start_app = start_app;
                self.args = build_args();
            }
            Message::RestartAppChanged(restart_app) => {
                CONFIG.write().unwrap().restart_app = restart_app;
                self.args = build_args();
            }
            Message::AppNameTypeChanged(app_name_type) => {
                CONFIG.write().unwrap().app_name_type = app_name_type;
                self.args = build_args();
            }
            Message::TimeLimitChanged(time_limit) => {
                if time_limit.trim().is_empty() {
                    CONFIG.write().unwrap().time_limit = None
                } else if let Ok(size) = time_limit.parse::<u32>() {
                    CONFIG.write().unwrap().time_limit = Some(size)
                };
                self.args = build_args();
            }
            Message::StayAwakeChanged(stay_awake) => {
                CONFIG.write().unwrap().stay_awake = stay_awake;
                self.args = build_args();
            }
            Message::DisableWindowChanged(disable_window) => {
                CONFIG.write().unwrap().disable_window = disable_window;
                self.args = build_args();
            }
            Message::BorderlessChanged(borderless) => {
                CONFIG.write().unwrap().borderless = borderless;
                self.args = build_args();
            }
            Message::AlwaysOnTopChanged(always_on_top) => {
                CONFIG.write().unwrap().always_on_top = always_on_top;
                self.args = build_args();
            }
            Message::FullscreenChanged(fullscreen) => {
                CONFIG.write().unwrap().fullscreen = fullscreen;
                self.args = build_args();
            }
            Message::DisableScreensaverChanged(disable_screensaver) => {
                CONFIG.write().unwrap().disable_screensaver = disable_screensaver;
                self.args = build_args();
            }
            Message::AdditionalArgsChanged(args) => {
                CONFIG.write().unwrap().additional_args = args;
                self.args = build_args();
            }

            Message::ArgsChanged(command) => {
                self.args = command;
            }
            Message::Resize(size) => {
                self.size = size;
            }
            Message::Run => {
                *ARGS.write().unwrap() = Some(self.args.clone());
                return window::get_latest().then(|id| close(id.unwrap()));
            }
        };
        ().into()
    }

    pub fn view(&self) -> Element<Message> {
        let config = CONFIG.try_read().unwrap();

        let config_section = column![
            components::exe_info(&config),
            components::connect_method(&config),
            horizontal_rule(style_default::Height::hr()),
            components::video(&config),
            horizontal_rule(style_default::Height::hr()),
            components::audio(&config),
            horizontal_rule(style_default::Height::hr()),
            components::performance(&config),
            horizontal_rule(style_default::Height::hr()),
            components::control(&config),
            horizontal_rule(style_default::Height::hr()),
            components::output(&config),
            horizontal_rule(style_default::Height::hr()),
            components::virtual_display(&config),
            horizontal_rule(style_default::Height::hr()),
            components::others(&config),
        ]
        .padding(style_default::Padding::page())
        .spacing(style_default::Spacing::general());

        container(
            column![
                scrollable(config_section).height(self.size.height - 100.0),
                components::action_section(self)
            ]
            .padding(style_default::Padding::container()),
        )
        .id(Id::new("page"))
        .into()
    }

    pub fn theme(&self) -> Theme {
        match dark_light::detect() {
            Mode::Dark => Theme::Dark,
            Mode::Light => Theme::Light,
            Mode::Default => Theme::Light,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|size| Message::Resize(size.1))
    }
}
