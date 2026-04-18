// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use nucleo_matcher::{Matcher, Config};
use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::Utf32String;
use freedesktop_desktop_entry::{Iter, DesktopEntry};
use std::path::Path;
use crate::Message;

pub struct Search {
    pub query: String,
    matcher: Matcher,
    entries: Vec<(String, String)>, // (name, exec)
}

impl Search {
    pub fn new() -> Self {
        let mut matcher = Matcher::new(Config::DEFAULT);
        let entries = Self::load_desktop_entries();

        Self {
            query: String::new(),
            matcher,
            entries,
        }
    }

    fn load_desktop_entries() -> Vec<(String, String)> {
        let mut entries = Vec::new();
        let iter = Iter::new(Some(&["/usr/share/applications", "\~/.local/share/applications"]));

        for path in iter {
            if let Ok(entry) = DesktopEntry::from_path(&path, None) {
                if let Some(name) = entry.name(None) {
                    if let Some(exec) = entry.exec() {
                        entries.push((name.to_string(), exec.to_string()));
                    }
                }
            }
        }
        entries
    }

    pub fn update(&mut self, msg: Message) -> Option<String> {
        match msg {
            Message::Search(query) => {
                self.query = query;
                None
            }
            Message::OpenLauncher => None,
            _ => None,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        // TODO: replace with your actual UI code
        iced::widget::text("Search goes here").into()
    }
}