// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::widget::{button, column, row, text, container, horizontal_space};
use iced::{Element, Length, Alignment};
use crate::Message;
use crate::position::LaunchPosition;

pub fn view(position: &crate::position::Position, drawer_type: Option<&str>) -> Element<Message> {
    let drawer_content = match drawer_type {
        Some("search") => column![
            text("Search").size(18),
            text("Start typing to search...").size(14),
        ].spacing(8),
        
        Some("pinned") => column![
            text("Pinned").size(18),
            button(text("Terminal")).on_press(Message::Search("alacritty".to_string())),
            button(text("Firefox")).on_press(Message::Search("firefox".to_string())),
            button(text("Files")).on_press(Message::Search("nautilus".to_string())),
        ].spacing(8),
        
        _ => text("Nothing here yet").size(14).into(),
    };

    let content = match position.current {
        LaunchPosition::TopLeft | LaunchPosition::BottomLeft => row! ,
        LaunchPosition::TopRight | LaunchPosition::BottomRight => row! ,
    };

    container(content)
        .width(Length::Fixed(400.0))
        .height(Length::Shrink)
        .padding(20)
        .into()
}