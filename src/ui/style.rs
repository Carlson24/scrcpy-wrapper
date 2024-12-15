pub mod style_default {
    pub struct Padding;

    impl Padding {
        pub fn input() -> iced::Padding {
            iced::Padding {
                top: 4.0,
                bottom: 4.0,
                left: 8.0,
                right: 8.0,
            }
        }

        pub fn page() -> iced::Padding {
            iced::Padding {
                top: 10.0,
                bottom: 10.0,
                left: 20.0,
                right: 20.0,
            }
        }
        
        pub fn container() -> iced::Padding {
            iced::Padding {
                top: 5.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            }
        }
    }

    pub struct Spacing;

    impl Spacing {
        pub fn general() -> f32 {
            10.0
        }
    }

    pub struct Size;

    impl Size {
        pub fn input() -> u16 {
            15
        }
        
        pub fn text_in_button() -> u16 {
            15
        }
        
        pub fn text_sub_title() -> u16 {
            20
        }
    }

    pub struct Height;

    impl Height {
        pub fn button() -> f32 {
            28.0
        }
        
        pub fn hr() -> f32 {
            10.0
        }
    }
}