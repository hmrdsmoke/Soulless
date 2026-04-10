// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::{Size, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockPosition {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl DockPosition {
    /// Returns the window size (fixed for now — easy to make configurable later)
    pub fn window_size(&self) -> Size {
        Size::new(460.0, 720.0)
    }

    /// Calculates ideal top-left position so the window "pops out" from the dock button area.
    pub fn window_position(&self) -> Point {
        let Size { width, height } = self.window_size();
        let screen_w = 2560.0; // TODO: read actual monitor geometry later
        let screen_h = 1440.0;
        let margin = 10.0;

        match self {
            DockPosition::BottomLeft => Point::new(margin, screen_h - height - margin),
            DockPosition::BottomRight => Point::new(screen_w - width - margin, screen_h - height - margin),
            DockPosition::TopLeft => Point::new(margin, margin),
            DockPosition::TopRight => Point::new(screen_w - width - margin, margin),
        }
    }

    /// Which direction the drawer content should slide in (for future animation).
    pub fn slide_direction(&self) -> SlideDirection {
        match self {
            DockPosition::BottomLeft | DockPosition::BottomRight => SlideDirection::Up,
            DockPosition::TopLeft | DockPosition::TopRight => SlideDirection::Down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SlideDirection {
    Up,
    Down,
    Left,
    Right,
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// use std::path::PathBuf; I am not using at the moment not sure if I will :: MRV

// === HISTORY ===
// DockPosition enum with BottomLeft, BottomRight, TopLeft, TopRight
// window_size and window_position methods
// slide_direction method for future animation

// === IN PROGRESS ===
// - #54 [ISSUE:position-001] Default toolbox position (make configurable later)
// - #55 [ISSUE:position-002] Pass position so search bar can be top/bottom
// - #56 [ISSUE:position-003] Toolbox = long rectangular pop-out window (your exact vision)
// - #57 [ISSUE:position-004] Support real multi-monitor geometry instead of hardcoded 2560x1440

// === DONE ===
// DockPosition enum with BottomLeft, BottomRight, TopLeft, TopRight
// window_size and window_position methods
// slide_direction method for future animation