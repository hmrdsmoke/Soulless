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
}

pub struct Search {
    pub query: String,
    matcher: Matcher,
    apps: Vec<(String, String)>,
}

impl Search {
    pub fn new() -> Self {
        let matcher = Matcher::new(Config::DEFAULT);
        let apps = load_desktop_entries();

        Self {
            query: String::new(),
            matcher,
            apps,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<String> {
        match message {
            Message::QueryChanged(q) => {
                self.query = q;
                None
            }
            Message::AppClicked(exec) => Some(exec),
        }
    }

    pub fn filtered_apps(&self) -> Vec<(String, String)> {
        if self.query.is_empty() {
            return self.apps.iter()
                .take(15)
                .map(|(n, e)| (n.clone(), e.clone()))
                .collect();
        }

        let pattern = Pattern::new(
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );

        let mut results: Vec<(u32, usize)> = self.apps.iter()
            .enumerate()
            .filter_map(|(i, (name, _))| {
                let haystack = Utf32String::from(name.as_str());
                pattern.score(haystack.as_slice(), &mut self.matcher.clone())
                    .map(|score| (score, i))
            })
            .collect();

        results.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        results.into_iter()
            .take(20)
            .map(|(_, i)| {
                let (name, exec) = &self.apps[i];
                (name.clone(), exec.clone())
            })
            .collect()
    }
}

fn load_desktop_entries() -> Vec<(String, String)> {
    let mut apps = vec![];
    for entry in Iter::new(freedesktop_desktop_entry::default_paths()) {
        if let Ok(entry) = DesktopEntry::from_path(&entry, Some(&[] as &[&str])) {
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
// add helper for search :: approval :: MRV

// === HISTORY ===
// Added private helper `load_desktop_entries()`
// Simplified apps storage to (String, String) tuple
// Different approach: on-the-fly Utf32String only during filtering
// Fixed all Utf32String method errors (as_slice, as_str, as_utf32str, etc.)
// Placeholder stripping moved to main.rs

// === IN PROGRESS ===
// (none - core search functionality is now stable)

// === DONE ===
// Added private helper `load_desktop_entries()`
// Simplified apps storage to (String, String) tuple
// Different approach: on-the-fly Utf32String only during filtering
// Fixed all Utf32String method errors (as_slice, as_str, as_utf32str, etc.)
// Placeholder stripping moved to main.rs
// Message enum derives Clone for iced 0.14 compatibility
// Fuzzy matching with nucleo-matcher (Pattern + Matcher)
// Desktop entry parsing with freedesktop-desktop-entry crate
// Added stable ID format support for board-sync workflow
// - Test if the sync workflow is working