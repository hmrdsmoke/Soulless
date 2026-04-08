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
        Self {
            query: String::new(),
        }
    }

    // Searches the ENTIRE library, not just current folder
    pub fn search<'a>(&self, entire_library: &'a , max_results: usize) -> Vec<&'a crate::App> {
        let query = self.query.trim().to_lowercase();

        if query.is_empty() {
            return entire_library.iter().take(max_results).collect();
        }

        let mut results = Vec::with_capacity(max_results);

        // Strict prefix first
        for app in entire_library {
            if app.name.to_lowercase().starts_with(&query) {
                results.push(app);
            }
        }

        // Then fuzzy to fill up to 200
        if results.len() < max_results {
            let remaining = max_results - results.len();
            let mut matcher = Matcher::new(Default::default());
            let pattern = Pattern::new(
                &query,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Fuzzy,
            );

            let mut fuzzy: Vec<(u32, &crate::App)> = entire_library.iter()
                .filter(|app| !results.iter().any(|r| r.name == app.name))
                .filter_map(|app| {
                    let haystack = Utf32String::from(app.name.as_str());
                    pattern.score(&haystack, &mut matcher)
                        .map(|score| (score, app))
                })
                .collect();

            fuzzy.sort_by(|a, b| b.0.cmp(&a.0));
            results.extend(fuzzy.into_iter().take(remaining).map(|(_, app)| app));
        }

        results.truncate(max_results);
        results
    }
}
