// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::{Size, Point};
// use std::path::PathBuf; I am not using at the moment not sure if I will :: MRV

/// Where the dock button lives — determines window position and drawer slide direction.
/// This makes Soulless feel like a native panel/dock launcher on Pop!_OS.
/// MRV: User can later configure this via settings drawer.
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
    /// On a typical 1920x1080 screen this puts it nicely in the corner.
    /// Future: read actual monitor geometry via winit/wayland for multi-monitor perfection.
    pub fn window_position(&self) -> Point {
        let Size { width, height } = self.window_size();
        let screen_w = 1920.0; // TODO: get real screen size
        let screen_h = 1080.0;
        let margin = 10.0; // gap from edge

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