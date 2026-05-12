// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use freedesktop_desktop_entry::DesktopEntry;
use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32String};
use std::fs;

#[derive(Clone)]
pub enum Message {
    QueryChanged(String),
    AppClicked(String),
    DrawerClicked(String),
    VaultClicked,
    SearchBarClicked,
}

#[derive(Clone, PartialEq, Eq)]
pub enum OpenDrawer {
    Search,
    Pinned(String),
    Vault,
}

#[derive(Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    lower_name: String,
    haystack: Utf32String,
}

pub struct Search {
    pub query: String,
    matcher: Matcher,
    all_apps: Vec<AppEntry>,
    filtered_apps: Vec<usize>,
    drawers: Vec<String>,
    pub show_search_results: bool,
    pub current_open_drawer: OpenDrawer,
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

        let mut search = Self {
            query: String::new(),
            matcher,
            all_apps,
            filtered_apps: Vec::new(),
            drawers,
            show_search_results: true, // MRV: launcher should open with search results ready
            current_open_drawer: OpenDrawer::Search, // MRV: launcher should open directly in search
        };

        search.recompute_results();
        search
    }

    pub fn update(&mut self, message: Message) -> Option<String> {
        match message {
            Message::QueryChanged(q) => {
                self.query = q;
                self.recompute_results();
                self.show_search_results = true;
                self.current_open_drawer = OpenDrawer::Search;
                None
            }
            Message::AppClicked(exec) => Some(exec),
            Message::DrawerClicked(name) => {
                self.current_open_drawer = OpenDrawer::Pinned(name);
                self.show_search_results = false;
                None
            }
            Message::VaultClicked => {
                self.current_open_drawer = OpenDrawer::Vault;
                self.show_search_results = false;
                None
            }
            Message::SearchBarClicked => {
                self.show_search_results = true;
                self.current_open_drawer = OpenDrawer::Search;
                None
            }
        }
    }

    pub fn filtered_apps(&self) -> &[usize] {
        &self.filtered_apps
    }

    pub fn app(&self, index: usize) -> Option<&AppEntry> {
        self.all_apps.get(index)
    }

    fn recompute_results(&mut self) {
        const MAX_RESULTS: usize = 200;
        const PREFIX_BONUS: u32 = 20_000;
        const SUBSTRING_BONUS: u32 = 10_000;

        if self.query.is_empty() {
            self.filtered_apps = (0..self.all_apps.len().min(MAX_RESULTS)).collect(); // MRV: show default ready results immediately at launcher open
            return;
        }

        let query_lower = self.query.to_lowercase();

        if self.query.chars().count() < 2 {
            self.filtered_apps = self
                .all_apps
                .iter()
                .enumerate()
                .filter(|(_, app)| app.lower_name.starts_with(&query_lower))
                .map(|(i, _)| i)
                .take(MAX_RESULTS)
                .collect();
            return;
        }

        let pattern = Pattern::new(
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );
        let mut matcher = self.matcher.clone();

        let mut scored: Vec<(u32, usize)> = self
            .all_apps
            .iter()
            .enumerate()
            .filter_map(|(i, app)| {
                if app.lower_name.starts_with(&query_lower) {
                    return Some((PREFIX_BONUS, i));
                }

                if app.lower_name.contains(&query_lower) {
                    return Some((SUBSTRING_BONUS, i));
                }

                let score = pattern.score(app.haystack.slice(..), &mut matcher);

                if let Some(score) = score {
                    eprintln!(
                        "FUZZY MATCH query={:?} app={:?} score={}",
                        self.query, app.name, score
                    );
                    Some((score, i))
                } else {
                    None
                }
            })
            .collect();

        scored.sort_unstable_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| self.all_apps[a.1].name.cmp(&self.all_apps[b.1].name))
        });

        self.filtered_apps = scored
            .into_iter()
            .take(MAX_RESULTS)
            .map(|(_, i)| i)
            .collect();
    }

    pub fn drawers(&self) -> &[String] {
        &self.drawers
    }
}

fn load_desktop_entries() -> Vec<AppEntry> {
    let mut apps = Vec::new();

    let home = dirs::home_dir().unwrap_or_default();

    let dirs = [
        "/usr/share/applications".to_string(),
        "/usr/local/share/applications".to_string(),
        format!("{}/.local/share/applications", home.display()),
        "/var/lib/flatpak/exports/share/applications".to_string(),
        format!(
            "{}/.local/share/flatpak/exports/share/applications",
            home.display()
        ),
    ];

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                    if let Ok(desktop) = DesktopEntry::from_path::<&str>(path, &[]) {
                        if let Some(name) = desktop.name::<&str>(&[]) {
                            if let Some(exec) = desktop.exec() {
                                let Some(icon) = desktop.icon().map(|i| i.to_string()) else {
                                    continue;
                                };

                                if should_skip_entry(&name, exec) {
                                    continue;
                                }

                                let clean_exec = crate::strip_desktop_placeholders(exec);
                                let name_str = name.to_string();

                                apps.push(AppEntry {
                                    lower_name: name_str.to_lowercase(),
                                    haystack: Utf32String::from(name_str.as_str()),
                                    name: name_str,
                                    exec: clean_exec,
                                    icon,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps
}

fn should_skip_entry(name: &str, exec: &str) -> bool {
    if exec.contains("%u") || exec.contains("%U") || exec.contains("%f") || exec.contains("%F") {
        return true;
    }

    let lower_name = name.to_lowercase();
    let lower_exec = exec.to_lowercase();

    let suspicious_terms = [
        "handler",
        "oauth",
        "daemon",
        "service",
        "portal",
        "settings",
        "setup",
        "configuration",
        "config",
        "wifi",
        "wi-fi",
        "network",
        "bluetooth",
        "gnome",
        "tweaks",
        "control center",
        "firmware",
        "drivers",
        "package installer",
        "software updater",
        "extensions",
    ];

    suspicious_terms
        .iter()
        .any(|term| lower_name.contains(term) || lower_exec.contains(term))
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
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