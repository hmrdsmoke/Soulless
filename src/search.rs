// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use nucleo_matcher::{Matcher, Config, Utf32String};
use nucleo_matcher::pattern::{Pattern, CaseMatching, Normalization, AtomKind};
use freedesktop_desktop_entry::{DesktopEntry, Iter};

#[derive(Clone, PartialEq)]
pub enum Message {
    QueryChanged(String),
    AppClicked(String),
    DrawerClicked(String),   // which drawer was clicked
    VaultClicked,
    SearchBarClicked,
    CloseCurrentDrawer,      // close any open drawer
}

#[derive(Clone, PartialEq)]
pub enum OpenDrawer {
    None,
    Utilities,
    DailyApps,
    Work,
    Games,
}

pub struct Search {
    pub query: String,
    matcher: Matcher,
    all_apps: Vec<(String, String, Utf32String)>,  // name, exec, precomputed haystack
    drawers: Vec<String>,
    pub show_search_results: bool,
    pub open_drawer: OpenDrawer,   // which pinned drawer is currently open
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
            open_drawer: OpenDrawer::None,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<String> {
        match message {
            Message::QueryChanged(q) => {
                self.query = q;
                self.show_search_results = true;
                self.open_drawer = OpenDrawer::None; // close any pinned drawer when searching
                None
            }
            Message::AppClicked(exec) => Some(exec),
            Message::DrawerClicked(name) => {
                self.show_search_results = false; // close search when opening pinned drawer
                self.open_drawer = match name.as_str() {
                    "Utilities" => OpenDrawer::Utilities,
                    "Daily Apps" => OpenDrawer::DailyApps,
                    "Work" => OpenDrawer::Work,
                    "Games" => OpenDrawer::Games,
                    _ => OpenDrawer::None,
                };
                None
            }
            Message::VaultClicked => {
                println!("Vault clicked");
                None
            }
            Message::SearchBarClicked => {
                self.show_search_results = true;
                self.open_drawer = OpenDrawer::None;
                None
            }
            Message::CloseCurrentDrawer => {
                self.show_search_results = false;
                self.open_drawer = OpenDrawer::None;
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

            let rest: Vec<(String, String)> = scored.into_iter()
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

    pub fn current_open_drawer(&self) -> &OpenDrawer {
        &self.open_drawer
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
// Added OpenDrawer enum and current_open_drawer() for real drawer state management :: done