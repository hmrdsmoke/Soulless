// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use crate::position::DockPosition;
use crate::search::Message as SearchMessage;
use iced::widget::{button, column, container, row, scrollable, space, text, text_input};
use iced::{Color, Element, Length};

pub fn view<'a>(
    search: &'a crate::search::Search,
    _position: &'a DockPosition,
) -> Element<'a, SearchMessage> {
    let search_bar = text_input("Click here to search all apps...", &search.query)
        .on_input(SearchMessage::QueryChanged)
        .padding(16)
        .size(18);

    // Main toolbox - pinned drawers + vault (fixed 460px width)
    let main_toolbox = column![
        container(search_bar).padding(16),
        column(search.drawers().iter().map(|name| {
            button(
                row![
                    text("📁").size(18),
                    space::horizontal().width(Length::Fixed(12.0)),
                    text(name.clone()).size(16),
                    space::horizontal().width(Length::Fill),
                    text("→").size(14)
                ]
                .align_y(iced::alignment::Vertical::Center)
                .padding(14),
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
        }))
        .spacing(6),
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

    // Search drawer - slides out to the side
    let search_drawer: Element<'a, SearchMessage> = if search.show_search_results {
        let results = scrollable(
            column(search.filtered_apps().into_iter().map(|(name, exec)| {
                button(
                    row![
                        text(name).size(16),
                        space::horizontal().width(Length::Fill),
                        text("→").size(14)
                    ]
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(12),
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
            }))
            .spacing(4),
        )
        .height(Length::Fill);

        row![
            main_toolbox,
            space::horizontal().width(Length::Fixed(12.0)),
            results
        ]
        .spacing(0)
        .width(Length::Shrink)
        .height(Length::Fill)
        .into()
    } else {
        main_toolbox.into()
    };

    search_drawer
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// Status is now an enum in iced 0.14 :: MRV
// 4 columns = nice square grid feel :: MRV
// Square grid for results (your requested "new square grid") :: MRV
// === DONE ===
// Top part (header + search) stays as the "toolbox top/handle"
// Search drawer now shows full list of all apps (no limit) when opened :: done
// Fixed iced 0.14 compatibility :: done
// Final layout: main toolbox stays fixed at 460px, search drawer slides out to the side :: done
// use std::path::PathBuf; I am not using at moment not sure if I will :: MRV
// removed # from start of line :: MRV
// fixed indexing :: MRV
// added for iced 0.14 compatibility :: MRV
// === IN PROGRESS ===
// (none for search.rs right now - core functionality is stable)
// === DONE ===
// Added private helper `load_desktop_entries()`
// Simplified apps storage to (String, String) tuple
// Different approach: on-the-fly Utf32String only during filtering
// Fixed all Utf32String method errors (as_slice, as_str, etc.)
// Placeholder stripping moved to main.rs
// Fixed .slice(..) for nucleo-matcher 0.3
// Eliminated per-item Matcher clone (reuse single mutable Matcher for zero-cost scoring) :: done
// Pinned to actual tested version nucleo-matcher = "0.3.1" and freedesktop-desktop-entry = "0.6.2" :: done
// All Cargo.toml deps now exact versions per Michael's new rule :: done
// Fixed DesktopEntry::from_path (takes PathBuf, locales = &[]) :: done
// Changed filtered_apps back to &self + clone Matcher once per query for iced 0.14 compatibility :: done
// Fixed locale type inference with `&[] as &[&str]` for freedesktop-desktop-entry 0.6.2 :: done
// Implemented exact search behavior: full alpha list on empty, top-10 strict prefix + fuzzy below :: done
// Separated drawers list from app search per final spec :: done
// Removed take(50) limit — now returns the complete list of all apps when search drawer opens :: done
// Simplified search logic — removed fragile skip_while, now clean prefix + fuzzy split :: done
// Replaced filtered_apps with your exact clean version (full list + prefix top-10 + fuzzy rest) :: done
// Added SearchBarClicked message to trigger drawer on click :: done
// Fixed unused_mut warning in filtered_apps (removed mut from rest) :: done
// Added get_app_by_index safety method to prevent crashes on results access :: done
// Pre-computed Utf32String for every app name at startup to eliminate repeated conversion cost :: done
// Fixed indexing bug in rest filter :: done
// Added OpenDrawer enum and current_open_drawer() for real drawer state management :: done
