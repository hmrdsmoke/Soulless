// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

// src/position.rs

use cosmic::cctk::sctk::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};
use cosmic::iced::platform_specific::runtime::wayland::layer_surface::SctkLayerSurfaceSettings;
use cosmic::iced::platform_specific::shell::commands::layer_surface::get_layer_surface;
use cosmic::iced::{Task, window};
use cosmic::iced::advanced::layout::Limits;
use crate::Message;

pub enum DockPosition { Left, Right }

pub enum SurfaceState {
    Hidden,
    Visible,
}

pub struct Position {
    pub window_id: window::Id,
    pub state: SurfaceState,
}

impl DockPosition {
    pub fn window_size(&self) -> iced::Size {
        iced::Size::new(460.0, 720.0)
    }
    pub fn window_position(&self) -> iced::Point {
        match self {
            DockPosition::Left => iced::Point::new(20.0, 200.0),
            DockPosition::Right => iced::Point::new(1500.0, 200.0),
        }
    }
}

impl Position {
    pub fn show(&mut self) -> Task<Message> {
        self.state = SurfaceState::Visible;
        get_layer_surface(SctkLayerSurfaceSettings {
            id: self.window_id,
            layer: Layer::Top,
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            anchor: Anchor::LEFT | Anchor::BOTTOM,
            namespace: "soulless-menu".into(),
            size: Some((Some(320), Some(620))),
            size_limits: Limits::NONE.min_height(400.0).max_height(620.0),
            exclusive_zone: 0,
            input_zone: Default::default(),
            output: Default::default(),
            margin: cosmic::iced::platform_specific::runtime::wayland::layer_surface::IcedMargin {
                bottom: 80,
                ..Default::default()
            },
        })
    }

    pub fn hide(&mut self) -> Task<Message> {
        self.state = SurfaceState::Hidden;
        // destroy_layer_surface logic can be added here later
        Task::none()
    }

    pub fn window_size(&self) -> iced::Size {
        iced::Size::new(460.0, 720.0)
    }

    pub fn window_position(&self) -> iced::Point {
        // Adjust these values based on your preferred dock position
        iced::Point::new(20.0, 200.0)
    }
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// use std::path::PathBuf; I am not using at moment not sure if I will :: MRV
// real monitor geometry detection (winit/wayland) :: working
// configurable dock position via settings :: working
// Basic Left position with centered vertical placement :: done
// === IN PROGRESS ===
// real monitor geometry detection (winit/wayland) :: in progress
// configurable dock position via settings :: in progress
