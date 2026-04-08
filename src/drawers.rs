// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::widget::{button, column, container, row, scrollable, text, Image};
use iced::{Element, Length, Color, Theme, Border};

# pub enum Message {
    AppClicked(String),  // app name or path
}

pub fn view() -> Element<Message> {
    let header = container(
        text("Soulless")
            .size(24)
            .style(Color::from_rgb(1.0, 0.95, 0.0)) // yellow
    )
    .padding(12)
    .style(|_| container::Style {
        background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
        ..Default::default()
    });

    let content = scrollable(
        column((0..50).map(|i| {  // placeholder for now
            button(
                row![
                    // Icon placeholder (we'll replace with real icons later)
                    text("📦").size(28),
                    text(format!("Folder {}", i)).size(16)
                ]
                .spacing(12)
                .align_y(iced::Alignment::Center)
            )
            .width(Length::Fill)
            .padding(12)
            .style(|theme, status| {
                let mut style = button::primary(theme, status);
                if status.hovered {
                    style.background = Some(Color::from_rgb(0.8, 0.2, 0.2).into()); // red hover
                    style.border = Border {
                        color: Color::from_rgb(0.9, 0.2, 0.2),
                        width: 2.0,
                        radius: 4.0.into(),
                    };
                }
                style
            })
            .into()
        }))
        .spacing(4)
    )
    .spacing(4);

    column! .spacing(0)
        .width(Length::Fixed(440.0))
        .height(Length::Fill)
        .into()
}
