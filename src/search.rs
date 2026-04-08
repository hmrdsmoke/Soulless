// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use nucleo_matcher::{Matcher, Config, Utf32String};
use nucleo_matcher::pattern::{Pattern, CaseMatching, Normalization, AtomKind};
use freedesktop_desktop_entry::{DesktopEntry, Iter};
// use std::path::PathBuf; I am not using at moment not sure if I will :: MRV

pub enum Message {    // removed # from start of line :: MRV
    QueryChanged(String),
    AppClicked(String),
}

pub struct Search {
    pub query: String,
    matcher: Matcher,
    apps: Vec<(String, String, Utf32String)>,
}

impl Search {
    pub fn new() -> Self {
        let mut apps = vec![];
        let matcher = Matcher::new(Config::DEFAULT);

        for entry in Iter::new(freedesktop_desktop_entry::default_paths()) {
            if let Ok(entry) = DesktopEntry::from_path(&entry, None) {
                if let Some(name) = entry.name(None) {
                    if let Some(exec) = entry.exec() {
                        let name_str = name.to_string();
                        let haystack = Utf32String::from(name_str.as_str());
                        apps.push((name_str, exec.to_string(), haystack));
                    }
                }
            }
        }

        apps.sort_by(|a, b| a.0.cmp(&b.0));

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
                .map(|(n, e, _)| (n.clone(), e.clone()))
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
            .filter_map(|(i, (_, _, haystack))| {
                pattern.score(haystack.as_ref(), &mut self.matcher.clone())
                    .map(|score| (score, i))
            })
            .collect();

        results.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        results.into_iter()
            .take(20)
            .map(|(_, i)| {
                let (name, exec, _) = &self.apps[i];  // fixed indexing :: MRV
                (name.clone(), exec.clone())
            })
            .collect()
    }
}