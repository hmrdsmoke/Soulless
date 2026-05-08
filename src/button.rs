// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::widget::image::Handle;
use iced::{
    Element, Length, Task,
    widget::{button, image},
};

#[derive(Debug, Clone)]
pub enum Message {
    OpenLauncher,
}

pub struct LauncherButton;

impl LauncherButton {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenLauncher => {
                println!("Launcher button clicked - Opening Soulless");
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        button(
            image(Handle::from_path("assets/icons/launcher.png"))
                .width(Length::Fixed(36.0))
                .height(Length::Fixed(36.0)),
        )
        .width(Length::Fixed(56.0))
        .height(Length::Fixed(56.0))
        .style(|_, _| button::Style {
            background: Some(iced::Color::from_rgb8(255, 215, 0).into()),
            border: iced::border::rounded(16),
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                offset: iced::Vector::new(3.0, 6.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        })
        .on_press(Message::OpenLauncher)
        .into()
    }
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// removed for now not sure if needed :: MRV
// needed for single-instance lock :: MRV
// your repo uses drawer.rs (singular) :: fixed :: MRV
// repo and local files now match changed drawer.rs to drawers.rs :: MRV
// removed # from start of line :: MRV
// Single-instance check — gives real launcher behavior (second launch activates :: MRV
// instead of spawning duplicate) :: MRV
// later replace with Unix socket signal to show/hide the window :: STILL NEEDS ::
// changed .window_position to .position this is the correct method name in iced 0.14 :: MRV
// end of change :: MRV
// Simple single-instance guard using XDG data dir + exclusive file lock. :: MRV
// Keeps startup extremely fast (sub-millisecond) and binary small. :: MRV
// This makes Soulless feel like a true system launcher, not a regular app. :: MRV
// we own the lock → sole instance :: MRV
// === IN PROGRESS ===
// click outside or Esc for launcher feel :: working
// borderless = native dock/pop-up feel :: working
// default toolbox position
// auto-close after launch (classic launcher behavior) :: done
// pass position so search bar can be top/bottom
// Toolbox = long rectangular pop-out window (your exact vision)
// click anywhere outside closes (real launcher feel)
// default yellow background with depth will be added later
// === DONE ===
// Single-instance check
// .always_on_top(true) not available in current builder style
// Added global subscription for Esc + click-outside handling
// Fixed Escape key check for iced 0.14 using your exact matches! syntax :: done
// Added `use dirs;` for dirs::data_dir() :: done
// Hooked up DockPosition to drawers::view so side-sliding works :: done
// Window now docks to Left or Right side correctly :: done
// Launcher button cleaned and simplified :: done
