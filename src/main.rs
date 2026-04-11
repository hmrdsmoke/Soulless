// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use nucleo_matcher::{Matcher, Config, Utf32String};
use nucleo_matcher::pattern::{Pattern, CaseMatching, Normalization, AtomKind};
use freedesktop_desktop_entry::{DesktopEntry, Iter};
use std::path::Path;

#[derive(Clone)]
pub enum Message {
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

        // Standard desktop file locations
        let paths = vec![
            Path::new("/usr/share/applications"),
            &dirs::home_dir().unwrap_or_default().join(".local/share/applications"),
        ];

        for base in paths {
            if base.exists() {
                for entry in Iter::new(std::iter::once(base.to_path_buf())) {
                    if let Ok(de) = DesktopEntry::from_path(&entry, None::<&[&str]>) {
                        if let (Some(name), Some(exec)) = (de.name(&[] as &[&str]), de.exec()) {
                            let name_str = name.to_string();
                            let haystack = Utf32String::from(name_str.as_str());
                            apps.push((name_str, exec.to_string(), haystack));
                        }
                    }
                }
            }
        }

        // Common binary locations for CLI tools
        let bin_paths = vec![
            dirs::home_dir().unwrap_or_default().join("bin"),
            dirs::home_dir().unwrap_or_default().join(".cargo/bin"),
            Path::new("/usr/local/bin").to_path_buf(),
            Path::new("/usr/bin").to_path_buf(),
        ];

        for bin_dir in bin_paths {
            if bin_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&bin_dir) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                let name_str = name.to_string();
                                let haystack = Utf32String::from(name_str.as_str());
                                let cmd = path.to_string_lossy().to_string();
                                apps.push((name_str, cmd, haystack));
                            }
                        }
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
                let (name, exec, _) = &self.apps[i];
                (name.clone(), exec.clone())
            })
            .collect()
    }
}

// === ALL YOUR ORIGINAL COMMENTS MOVED TO THE BOTTOM (preserved exactly) ===
 // fixed import syntax :: MRV
 // changed from freedesktop_desktop_entry:: to freedesktop-desktop-entry 0.8:: :: MRV
 // use std::path::PathBuf; :: I am not using at moment not sure if I will :: MRV
 // removed # from start of line :: MRV
 // locales: empty slice :: MRV
 // changed from if let Some(name) = entry.name(None) { to if let Some(name) = entry.name(&[] as &[&str]) { : gives local empty slice :: MRV
 // fixed indexing :: MRV
 // needs &Utf32Str, not &Utf32String :: MRV