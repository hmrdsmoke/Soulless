// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};

pub enum Message {
    AppClicked(String),
}

pub fn view() -> Element<Message> {
    let yellow_header = container(
        text("Soulless")
            .size(28)
            .style(Color::from_rgb(1.0, 0.95, 0.0))   // bright yellow
    )
    .padding(16)
    .center_x(Length::Fill)
    .style(|_| container::Style {
        background: Some(Color::from_rgb(0.12, 0.12, 0.12).into()),
        ..Default::default()
    });

    let app_buttons = scrollable(
        column(
            (0..30).map(|i| {   // placeholder - we'll replace with real apps later
                button(
                    row![
                        text("📦").size(32),                    // icon
                        text(format!("App {}", i)).size(18)     // text
                    ]
                    .spacing(16)
                    .align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .padding(14)
                .style(|theme, status| {
                    let mut s = button::primary(theme, status);
                    if status.hovered {
                        s.background = Some(Color::from_rgb(0.85, 0.15, 0.15).into()); // red hover
                    }
                    s
                })
                .into()
            })
        )
        .spacing(6)
    );

    column![
        yellow_header,
        app_buttons
    ]
    .width(Length::Fixed(460.0))      // nice narrow width
    .height(Length::FillPortion(3))   // roughly 1/3 of the screen
    .into()
}
