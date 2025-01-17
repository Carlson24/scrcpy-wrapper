#[macro_export]
macro_rules! define_component {
    ($name:ident, $closure:expr) => {
        pub fn $name<'a>(
            config: &std::sync::RwLockReadGuard<'static, Box<$crate::config::Config>>, win_main: &$crate::ui::WinMain
        ) -> iced::Element<'a, $crate::ui::Message, iced::Theme, iced::Renderer> {
            let func: fn(
                config: &std::sync::RwLockReadGuard<'static, Box<$crate::config::Config>>, win_main: &$crate::ui::WinMain
            ) -> iced::Element<'a, $crate::ui::Message, iced::Theme, iced::Renderer> = $closure;
            func(config, win_main)
        }
    };
}

// #[macro_export]
// macro_rules! component {
//     ($name:ident) => {
//         $crate::ui::components::$name(config, win_main)
//     };
//     ($name: ident, $config: expr, $win_main: expr) => {
//         $crate::ui::components::$name($config, $win_main)
//     };
// }
