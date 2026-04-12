// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

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
    all_apps: Vec<(String, String)>,
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
            return self.all_apps.clone();
        }

        let prefix = self.query.to_lowercase();

        let mut top: Vec<(String, String)> = self.all_apps.iter()
            .filter(|(name, _)| name.to_lowercase().starts_with(&prefix))
            .take(10)
            .map(|(n, e)| (n.clone(), e.clone()))
            .collect();
        top.sort_by(|a, b| a.0.cmp(&b.0));

        let pattern = Pattern::new(
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );
        let mut matcher = self.matcher.clone();

        let mut scored: Vec<(u32, usize)> = self.all_apps.iter()
            .enumerate()
            .filter_map(|(i, (name, _))| {
                let haystack = Utf32String::from(name.as_str());
                pattern.score(haystack.slice(..), &mut matcher)
                    .map(|score| (score, i))
            })
            .collect();

        scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut rest: Vec<(String, String)> = scored.into_iter()
            .filter(|(_, i)| !top.iter().any(|(n, _)| n == &self.all_apps[*i].0))
            .map(|(_, i)| {
                let (name, exec) = &self.all_apps[i];
                (name.clone(), exec.clone())
            })
            .collect();

        top.extend(rest);
        top
    }

    pub fn drawers(&self) -> &[String] {
        &self.drawers
    }
}

fn load_desktop_entries() -> Vec<(String, String)> {
    let mut apps = vec![];

    for entry in Iter::new(freedesktop_desktop_entry::default_paths()) {
        if let Ok(entry) = DesktopEntry::from_path(entry, &[] as &[&str]) {
            if let Some(name) = entry.name::<&str>(&[]) {
                if let Some(exec) = entry.exec() {
                    let name_str = name.to_string();
                    apps.push((name_str, exec.to_string()));
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