// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::{Element, Task, Theme, widget::{column, container, text_input}};
use crate::button::Button;
use crate::position::{Position, LaunchPosition};
use crate::search::Search;
use crate::drawers;

pub enum Message {
    OpenLauncher,
    CloseLauncher,
    Search(String),
    OpenDrawer(String),
}

pub struct Soulless {
    button: Button,
    position: Position,
    search: Search,
    current_drawer: Option<String>,
    is_open: bool,
}

impl Soulless {
    fn new() -> (Self, Task<Message>) {
        (Self {
            button: Button::new(),
            position: Position::new(LaunchPosition::TopRight),
            search: Search::new(),
            current_drawer: None,
            is_open: false,
        }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Open
            Message::OpenLauncher => {
                self.is_open = true;
                self.current_drawer = Some("pinned".to_string());
                self.position.show()
            }
            Message::Search(query) => {
                self.search.update(query);
                if !query.is_empty() {
                    self.current_drawer = Some("search".to_string());
                }
                Task::none()
            }
            Message::OpenDrawer(drawer) => {
                self.current_drawer = Some(drawer);
                Task::none()
            }
            Message::CloseLauncher => {
                self.is_open = false;
                self.current_drawer = None;
                iced::exit()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        if !self.is_open {
            return self.button.view().map(|_| Message::OpenLauncher);
        }

        let search_bar = text_input("Search...", &self.search.query)
            .on_input(Message::Search)
            .size(18);

        let drawer = drawers::view(&self.position, self.current_drawer.as_deref());

        container(column! )
            .width(Length::Fixed(420.0))
            .height(Length::Shrink)
            .into()
    }
}