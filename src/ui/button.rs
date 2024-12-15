use crate::ui::style::style_default;
use iced::widget::PickList;
use iced::{Center, Length};
use std::fmt::Display;

pub trait ButtonState {
    fn states() -> Vec<Self>
    where
        Self: Sized;
}

pub struct StateButton<'a, T: ButtonState + Sized + Clone + PartialEq + Display, Message> {
    value: T,
    on_press: Box<dyn Fn(T) -> Message + 'a>,
}

impl<'a, T, Message: 'a + Clone> StateButton<'a, T, Message>
where
    T: 'a + ButtonState + Sized + Clone + PartialEq + Display,
{
    fn _on_press(&self) -> Message {
        let states = T::states();
        let i = states.iter().position(|s| s == &self.value);
        let i = if let Some(i) = i {
            (i + 1) % states.len()
        } else {
            0
        };
        (self.on_press)(T::states()[i].clone())
    }

    pub fn button(
        value: T,
        on_press: impl Fn(T) -> Message + 'a,
    ) -> iced::widget::Button<'a, Message> {
        let c = Self {
            value,
            on_press: Box::new(on_press),
        };
        iced::widget::button(
            iced::widget::text(c.value.to_string())
                .align_x(Center)
                .align_y(Center)
                .size(style_default::Size::text_in_button()),
        )
        .on_press_with(move || c._on_press())
        .style(iced::widget::button::secondary)
        .height(style_default::Height::button())
        .width(Length::Shrink)
    }

    pub fn pick_list(
        value: T,
        on_press: impl Fn(T) -> Message + 'a,
    ) -> PickList<'a, T, Vec<T>, T, Message> {
        iced::widget::pick_list(T::states(), Some(value.clone()), on_press)
            .text_size(style_default::Size::text_in_button())
            .padding(style_default::Padding::input())
            .width(Length::Shrink)
    }
}
