use cosmic::iced::widget::button;
use cosmic::iced::{Element, Length};
use crate::Message;

pub struct Button;

impl Button {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        let icon = cosmic::widget::image("assets/profile.jpg")
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(48.0));

        button(icon)
            .on_press(Message::OpenLauncher)
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(48.0))
            .into()
    }
}