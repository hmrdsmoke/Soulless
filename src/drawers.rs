// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length, Padding};
use crate::search::Message as SearchMessage;

pub fn view(search: &crate::search::Search) -> Element<SearchMessage> {
    let header = container(
        text("Soulless")
            .size(32)
            .style(|_| Color::from_rgb(1.0, 0.95, 0.0))
    )
    .padding(20)
    .center_x(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::color!(0x1e, 0x1e, 0x1e).into()),
        border: iced::border::rounded(8),
        ..Default::default()
    });

    let search_bar = text_input("Type to search...", &search.query)
        .on_input(SearchMessage::QueryChanged)
        .padding(14)
        .size(18)
        .style(|theme, status| text_input::default(theme, status));

    let results = scrollable(
        column(
            search.filtered_apps().into_iter().map(|(name, exec)| {
                button(
                    row![] .spacing(16)
                    .align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .padding(14)
                .on_press(SearchMessage::AppClicked(exec))
                .style(|theme, status| {
                    let mut s = button::primary(theme, status);
                    if status.hovered {
                        s.background = Some(iced::color!(0xd4, 0x22, 0x22).into());
                    }
                    s
                })
                .into()
            })
        )
        .spacing(4)
        .padding(Padding::new(8.0))
    )
    .height(Length::Fill);

    column![
        header,
        container(search_bar).padding(16),
        results
    ]
    .spacing(8)
    .width(Length::Fixed(460.0))
    .height(Length::Fill)
    .into()
}