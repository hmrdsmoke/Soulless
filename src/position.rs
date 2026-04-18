use iced::platform_specific::runtime::wayland::layer_surface::{SctkLayerSurfaceSettings, Layer, Anchor};
use iced::platform_specific::shell::commands::layer_surface::get_layer_surface;
use iced::{Task, window};
use crate::Message;

# pub enum LaunchPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct Position {
    pub current: LaunchPosition,
}

impl Position {
    pub fn new() -> Self {
        Self {
            current: LaunchPosition::BottomRight, // default
        }
    }

    pub fn set_position(&mut self, pos: LaunchPosition) {
        self.current = pos;
    }

    pub fn show(&self) -> Task<Message> {
        let (anchor, x, y) = match self.current {
            LaunchPosition::TopLeft => (Anchor::TOP | Anchor::LEFT, 20, 80),
            LaunchPosition::TopRight => (Anchor::TOP | Anchor::RIGHT, -20, 80),
            LaunchPosition::BottomLeft => (Anchor::BOTTOM | Anchor::LEFT, 20, -80),
            LaunchPosition::BottomRight => (Anchor::BOTTOM | Anchor::RIGHT, -20, -80),
        };

        get_layer_surface(SctkLayerSurfaceSettings {
            layer: Layer::Top,
            anchor,
            size: Some((Some(420), Some(520))),
            position: Some((x, y)),
            exclusive_zone: -1,
            ..Default::default()
        })
    }
}