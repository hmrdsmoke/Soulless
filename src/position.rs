// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::{Size, Point};

// use std::path::PathBuf; I am not using at moment not sure if I will :: MRV

/// Where the dock button lives — determines window position and drawer slide direction.
/// This makes Soulless feel like a native panel/dock launcher on Pop!_OS.
/// MRV: User can later configure this via settings drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockPosition {
    Left,   // launcher docks to left side
    Right,  // launcher docks to right side
}

impl DockPosition {
    /// Returns the window size (fixed for now — easy to make configurable later)
    pub fn window_size(&self) -> Size {
        Size::new(460.0, 720.0)
    }

    /// Calculates ideal top-left position so the window "pops out" from the dock button area.
    pub fn window_position(&self) -> Point {
        let Size { width, height } = self.window_size();
        let margin = 10.0;

        // Simple fallback for now (can be improved with real monitor detection later)
        let screen_w = 1920.0;
        let screen_h = 1080.0;

        match self {
            DockPosition::Left => Point::new(margin, (screen_h - height) / 2.0),   // centered vertically on left
            DockPosition::Right => Point::new(screen_w - width - margin, (screen_h - height) / 2.0),
        }
    }

    /// Which direction the drawer content should slide in.
    pub fn slide_direction(&self) -> SlideDirection {
        match self {
            DockPosition::Left => SlideDirection::Right,  // drawer slides out to the right
            DockPosition::Right => SlideDirection::Left,  // drawer slides out to the left
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SlideDirection {
    Left,
    Right,
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// use std::path::PathBuf; I am not using at moment not sure if I will :: MRV

// === IN PROGRESS ===
// real monitor geometry detection (winit/wayland) :: working
// configurable dock position via settings :: working

// === DONE ===
// Basic Left/Right positions with centered vertical placement :: done
// slide_direction() returns correct direction for side slide :: done
// window_position() now uses simple screen size instead of hardcoded 2560x1440 :: done