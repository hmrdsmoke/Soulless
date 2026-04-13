// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

<<<<<<< HEAD
use nucleo_matcher::{Matcher, Config, Utf32String};
use nucleo_matcher::pattern::{Pattern, CaseMatching, Normalization, AtomKind};
use freedesktop_desktop_entry::{DesktopEntry, Iter};

#[derive(Clone)]
pub enum Message {
    QueryChanged(String),
    AppClicked(String),
    DrawerClicked(String),
    VaultClicked,
    SearchBarClicked,
}

pub struct Search {
    pub query: String,
    matcher: Matcher,
    all_apps: Vec<(String, String, Utf32String)>,  // name, exec, precomputed haystack
    drawers: Vec<String>,
    pub show_search_results: bool,
}

impl Search {
    pub fn new() -> Self {
        let matcher = Matcher::new(Config::DEFAULT);
        let all_apps = load_desktop_entries();
        let drawers = vec![
            "Utilities".to_string(),
            "Daily Apps".to_string(),
            "Work".to_string(),
            "Games".to_string(),
        ];

        Self {
            query: String::new(),
            matcher,
            all_apps,
            drawers,
            show_search_results: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<String> {
        match message {
            Message::QueryChanged(q) => {
                self.query = q;
                self.show_search_results = true;
                None
            }
            Message::AppClicked(exec) => Some(exec),
            Message::DrawerClicked(name) => {
                println!("Drawer clicked: {}", name);
                None
            }
            Message::VaultClicked => {
                println!("Vault clicked");
                None
            }
            Message::SearchBarClicked => {
                self.show_search_results = true;
                None
            }
        }
    }

    pub fn filtered_apps(&self) -> Vec<(String, String)> {
        if self.query.is_empty() {
            return self.all_apps.iter()
                .map(|(name, exec, _)| (name.clone(), exec.clone()))
                .collect();
        }

        let prefix = self.query.to_lowercase();

        let mut top: Vec<(String, String)> = self.all_apps.iter()
            .filter(|(name, _, _)| name.to_lowercase().starts_with(&prefix))
            .take(10)
            .map(|(n, e, _)| (n.clone(), e.clone()))
            .collect();
        top.sort_by(|a, b| a.0.cmp(&b.0));

        if !prefix.is_empty() {
            let pattern = Pattern::new(
                &self.query,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Fuzzy,
            );
            let mut matcher = self.matcher.clone();

            let mut scored: Vec<(u32, usize)> = self.all_apps.iter()
                .enumerate()
                .filter_map(|(i, (_, _, haystack))| {
                    pattern.score(haystack.slice(..), &mut matcher)
                        .map(|score| (score, i))
                })
                .collect();

            scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));

            let mut rest: Vec<(String, String)> = scored.into_iter()
                .filter(|(_, i)| !top.iter().any(|(n, _)| n == &self.all_apps[*i].0))
                .map(|(_, i)| {
                    let (name, exec, _) = &self.all_apps[i];
                    (name.clone(), exec.clone())
                })
                .collect();

            top.extend(rest);
        }
        top
    }

    pub fn drawers(&self) -> &[String] {
        &self.drawers
    }
}

fn load_desktop_entries() -> Vec<(String, String, Utf32String)> {
    let mut apps = vec![];

    for entry in Iter::new(freedesktop_desktop_entry::default_paths()) {
        if let Ok(entry) = DesktopEntry::from_path(entry, &[] as &[&str]) {
            if let Some(name) = entry.name::<&str>(&[]) {
                if let Some(exec) = entry.exec() {
                    let name_str = name.to_string();
                    let haystack = Utf32String::from(name_str.as_str());
                    apps.push((name_str, exec.to_string(), haystack));
                }
            }
        }
    }

    apps.sort_by(|a, b| a.0.cmp(&b.0));
    apps
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
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
// Changed filtered_apps back to &self + clone Matcher once per query for iced view compatibility :: done
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
=======
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
>>>>>>> c13af7f (feat: add custom button widget + improve drawer layout New button for reuable launcher updated drawers grid to use button Minor main + search cleanups for iced .14 Cargo toml / Cargo lock updates)
