// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use nucleo_matcher::{Matcher, pattern::{AtomKind, CaseMatching, Normalization, Pattern}};
use nucleo_matcher::Utf32String;

pub struct Search {
    pub query: String,
}

impl Search {
    pub fn new() -> Self {
        Self { query: String::new() }
    }

    pub fn search<'a>(&self, all_apps: &'a , max_results: usize) -> Vec<&'a crate::App> {
        let query = self.query.trim().to_lowercase();
        let mut results = Vec::with_capacity(max_results);

        if query.is_empty() {
            return all_apps.iter().take(max_results).collect();
        }

        let char_count = query.len();

        // 1–3 characters: First 10 strict, then 75/25 blend
        if char_count <= 3 {
            // Strict first (top 10)
            for app in all_apps {
                if app.name.to_lowercase().starts_with(&query) && results.len() < 10 {
                    results.push(app);
                }
            }

            // 75% strict / 25% fuzzy blend for the rest
            let remaining = max_results - results.len();
            let strict_count = (remaining as f32 * 0.75) as usize;

            // More strict matches
            for app in all_apps {
                if !results.contains(&app) && app.name.to_lowercase().starts_with(&query) {
                    if results.len() < max_results {
                        results.push(app);
                    }
                }
            }

            // Fuzzy to fill remaining 25%
            if results.len() < max_results {
                let fuzzy_needed = max_results - results.len();
                let mut matcher = Matcher::new(Default::default());
                let pattern = Pattern::new(&query, CaseMatching::Smart, Normalization::Smart, AtomKind::Fuzzy);

                let mut fuzzy_results: Vec<(u32, &crate::App)> = all_apps.iter()
                    .filter(|app| !results.iter().any(|r| r.name == app.name))
                    .filter_map(|app| {
                        let haystack = Utf32String::from(app.name.as_str());
                        pattern.score(&haystack, &mut matcher).map(|score| (score, app))
                    })
                    .collect();

                fuzzy_results.sort_by(|a, b| b.0.cmp(&a.0));
                results.extend(fuzzy_results.into_iter().take(fuzzy_needed).map(|(_, app)| app));
            }
        } 
        // 4+ characters: 100% strict until no more found, then fuzzy
        else {
            for app in all_apps {
                if app.name.to_lowercase().starts_with(&query) {
                    results.push(app);
                }
            }

            if results.len() < max_results {
                let remaining = max_results - results.len();
                let mut matcher = Matcher::new(Default::default());
                let pattern = Pattern::new(&query, CaseMatching::Smart, Normalization::Smart, AtomKind::Fuzzy);

                let mut fuzzy: Vec<(u32, &crate::App)> = all_apps.iter()
                    .filter(|app| !results.iter().any(|r| r.name == app.name))
                    .filter_map(|app| {
                        let haystack = Utf32String::from(app.name.as_str());
                        pattern.score(&haystack, &mut matcher).map(|score| (score, app))
                    })
                    .collect();

                fuzzy.sort_by(|a, b| b.0.cmp(&a.0));
                results.extend(fuzzy.into_iter().take(remaining).map(|(_, app)| app));
            }
        }

        results.truncate(max_results);
        results
    }
}
