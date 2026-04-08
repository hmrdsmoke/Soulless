// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.
//
// Soulless — Replacement app launcher for Cosmic

use iced::{Element, Size, Task, Theme};

mod drawer;
mod search;
mod widgets;
mod hmrdvault;

# pub enum Message {
    ToggleDrawer,
    Drawer(drawer::Message),
    Search(search::Message),
    Vault(hmrdvault::Message),
}

fn main() -> iced::Result {
    iced::application(Soulless::new, Soulless::update, Soulless::view)
        .window_size(Size::new(420.0, 680.0))
        .theme(Soulless::theme)
        .centered()
        .run()
}

struct Soulless {
    drawer_open: bool,
}

impl Soulless {
    fn new() -> (Self, Task<Message>) {
        (Self { drawer_open: false }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleDrawer => {
                self.drawer_open = !self.drawer_open;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        widgets::view(self)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
