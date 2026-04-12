// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::widget::{button, column, container, row, scrollable, text, text_input, space};
use iced::{Color, Element, Length};
use crate::search::Message as SearchMessage;
use crate::position::{DockPosition, SlideDirection};

pub fn view<'a>(
    search: &'a crate::search::Search,
    position: &'a DockPosition,
) -> Element<'a, SearchMessage> {
    let search_bar = text_input("Click here to search all apps...", &search.query)
        .on_input(SearchMessage::QueryChanged)
        .padding(16)
        .size(18);

    // Main toolbox - pinned drawers + vault (fixed 460px width, never changes)
    let main_toolbox = column![
        container(search_bar).padding(16),

        // Pinned drawers
        column(
            search.drawers().iter().map(|name| {
                button(
                    row![
                        text("📁").size(18),
                        space::horizontal().width(Length::Fixed(12.0)),
                        text(name.clone()).size(16),
                        space::horizontal().width(Length::Fill),
                        text("→").size(14)
                    ]
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(14)
                )
                .width(Length::Fill)
                .height(Length::Fixed(58.0))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::from_rgb8(40, 40, 45).into()),
                    border: iced::border::rounded(8),
                    ..Default::default()
                })
                .on_press(SearchMessage::DrawerClicked(name.clone()))
                .into()
            })
        )
        .spacing(6),

        // Vault at bottom
        container(
            button(
                row![
                    text("🔒").size(20),
                    space::horizontal().width(Length::Fixed(12.0)),
                    text("Vault (Secure Folder)").size(16)
                ]
                .align_y(iced::alignment::Vertical::Center)
                .padding(14)
            )
            .width(Length::Fill)
            .height(Length::Fixed(68.0))
            .style(|_theme, _status| button::Style {
                background: Some(Color::from_rgb8(28, 28, 38).into()),
                border: iced::border::rounded(8),
                ..Default::default()
            })
            .on_press(SearchMessage::VaultClicked)
        )
        .padding(16)
    ]
    .spacing(8)
    .width(Length::Fixed(460.0))
    .height(Length::Fill);

    // Search drawer - slides out to the side when active
    let search_drawer: Element<'a, SearchMessage> = if search.show_search_results {
        let results = scrollable(
            column(
                search.filtered_apps().into_iter().map(|(name, exec)| {
                    button(
                        row![
                            text(name).size(16),
                            space::horizontal().width(Length::Fill),
                            text("→").size(14)
                        ]
                        .align_y(iced::alignment::Vertical::Center)
                        .padding(12)
                    )
                    .width(Length::Fill)
                    .height(Length::Fixed(52.0))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::from_rgb8(35, 35, 40).into()),
                        border: iced::border::rounded(6),
                        ..Default::default()
                    })
                    .on_press(SearchMessage::AppClicked(exec))
                    .into()
                })
            )
            .spacing(4)
        )
        .height(Length::Fill);

        match position.slide_direction() {
            SlideDirection::Left => row![results, space::horizontal().width(Length::Fixed(12.0))].into(),
            SlideDirection::Right => row![space::horizontal().width(Length::Fixed(12.0)), results].into(),
            _ => column![results].into(),
        }
    } else {
        space::horizontal().width(Length::Shrink).into()
    };

    // Final layout: fixed main toolbox + sliding search drawer
    row![
        main_toolbox,
        search_drawer
    ]
    .spacing(0)
    .width(Length::Shrink)
    .height(Length::Fill)
    .into()
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// Status is now an enum in iced 0.14 :: MRV
// 4 columns = nice square grid feel :: MRV
// Square grid for results (your requested "new square grid") :: MRV

// :: In Progress
// :: The slide out square drawer where your daily apps live
// :: Not crashing when clicked due to a pop os window error
// :: Style colors shadowing and shading background icons
// :: Make launcher sharp square by default (option later for Pop!_OS style)

// === DONE ===
// Top part (header + search) stays as the "toolbox top/handle"
// Results now use a square grid (4 columns, fixed 100x100 buttons) as you requested
// Added subtle button background for better visual depth
// Fixed button style closure to match iced 0.14 (|theme, status|)
// Fixed grid .width() to take f32 (460.0) instead of Length::Fill
// Updated back to &search because filtered_apps is now &self again :: done
// Removed unnecessary .clone() on exec — now moved directly into AppClicked :: done
// Changed main view to show only drawer buttons (no apps) per final spec :: done
// Added dedicated Vault section at bottom with locked button stub :: done
// Updated to show pinned drawers in main view + separate search results behavior :: done
// Search results drawer now appears when search bar is clicked or typed in :: done
// Search drawer now shows full list of all apps (no limit) when opened :: done
// Fixed iced 0.14 compatibility: used space::horizontal / space::vertical :: done
// Used DockPosition.slide_direction() to make search drawer slide LEFT or RIGHT of the main toolbox :: done
// Final layout: main toolbox stays fixed at 460px, search drawer slides out to the side without resizing main toolbox :: done