// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::widget::{button, column, container, grid, scrollable, text, text_input};
use iced::{Color, Element, Length};
use crate::search::Message as SearchMessage;

pub fn view(search: &crate::search::Search) -> Element<'_, SearchMessage> {
    let search_bar = text_input("Type to search apps...", &search.query)
        .on_input(SearchMessage::QueryChanged)
        .padding(14)
        .size(18);

    let results_grid = grid(
        search.filtered_apps().into_iter().map(|(name, exec)| {
            button(
                text(name)
                    .size(14)
                    .center()
            )
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0))
            .style(|_| button::Style {
                border: iced::border::rounded(4), // sharp square look (small radius)
                ..Default::default()
            })
            .on_press(SearchMessage::AppClicked(exec.clone()))
            .into()
        })
    )
    .width(Length::Fill)
    .spacing(12)
    .columns(4);

    let results = scrollable(results_grid)
        .height(Length::Fill);

    column![
        container(search_bar).padding(16),
        results
    ]
    .spacing(8)
    .width(Length::Fixed(460.0))
    .height(Length::Fill)
    .into()
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// Status is now an enum in iced 0.14 :: MRV
// 4 columns = nice square grid feel :: MRV
// Square grid for results (your requested "new square grid") :: MRV

// === HISTORY ===
// NEW CHANGE (2026-04-10 by Harper under Grok review — 6th edit to drawers.rs):
// Improved the square grid based on your feedback ("needs more but good start").
// NEW CHANGE (2026-04-10 by Harper under Grok review — 5th edit to drawers.rs):
// Fixed compile error (E0277) on grid.width(Length::Fill).
// NEW CHANGE (2026-04-10 by Harper under Grok review — 4th edit to drawers.rs):
// Started implementing your real drawer vision: top part stays as toolbox handle, results use square grid.

// === IN PROGRESS ===
// - #44 [ISSUE:drawer-001] The slide out square drawer where your daily apps live
// - #45 [ISSUE:drawer-002] Not crashing when clicked due to a pop os window error
// - #46 [ISSUE:drawer-003] Style colors shadowing and shading background icons
// - #47 [ISSUE:drawer-004] Make launcher sharp square by default (option later for Pop!_OS style)

// === DONE ===
// Top part (header + search) stays as the "toolbox top/handle"
// Results now use a square grid (4 columns, fixed 100x100 buttons) as you requested
// Removed header per your request
// Sharp square styling with border::rounded(4)