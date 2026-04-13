// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

<<<<<<< HEAD
use iced::{widget::button, Element, Length, Task, Theme, window};
use iced::widget::text;

pub fn main() -> iced::Result {
    iced::application(DockButton::new, DockButton::update, DockButton::view)
        .window_size(Size::new(48.0, 48.0))           // small square button
        .position(window::Position::Specific(Point::new(50.0, 50.0))) // change to your preferred spot
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .theme(DockButton::theme)
        .run()
}

struct DockButton;

impl DockButton {
    fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        // TODO: send signal to main launcher to show window (Unix socket later)
        println!("Dock button clicked - opening Soulless launcher");
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        button(
            text("🔥")  // your icon - change to whatever you want
                .size(24)
                .center()
        )
        .width(Length::Fixed(48.0))
        .height(Length::Fixed(48.0))
        .style(|_theme, _status| button::Style {
            background: Some(iced::Color::from_rgb8(255, 215, 0).into()), // yellow like you like
            border: iced::border::rounded(12),
            ..Default::default()
        })
        .on_press(Message::Clicked)
        .into()
    }

    fn theme(_: &Self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
enum Message {
    Clicked,
=======
use iced::{widget::{button, image}, Element, Length, Task, Theme};
use iced::widget::image::Handle;

#[derive(Debug, Clone)]
pub enum Message {
    OpenLauncher,
}

pub struct DockButton;

impl DockButton {
    pub fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenLauncher => {
                println!("Dock button clicked - Opening Soulless launcher");
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        button(
            image(Handle::from_path("assets/icons/launcher.png"))
                .width(Length::Fixed(36.0))
                .height(Length::Fixed(36.0))
        )
        .width(Length::Fixed(56.0))
        .height(Length::Fixed(56.0))
        .style(|_, _| button::Style {
            background: Some(iced::Color::from_rgb8(255, 215, 0).into()),
            border: iced::border::rounded(16),
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                offset: iced::Vector::new(3.0, 6.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        })
        .on_press(Message::OpenLauncher)
        .into()
    }
>>>>>>> c13af7f (feat: add custom button widget + improve drawer layout New button for reuable launcher updated drawers grid to use button Minor main + search cleanups for iced .14 Cargo toml / Cargo lock updates)
}