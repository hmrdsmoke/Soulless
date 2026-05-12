// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use cosmic::{
    app::Core,
    applet::CosmicAppletHelper,
    iced::{self, Element, Length, Task},
    widget,
};

/// Messages for the Soulless panel applet.
#[derive(Debug, Clone)]
pub enum Message {
    /// User pressed the applet button — launch the Soulless launcher.
    LaunchSoulless,
    /// Launch subprocess completed (unused for now).
    Launched,
}

/// Soulless COSMIC panel applet.
pub struct SoullessApplet {
    core: Core,
    helper: CosmicAppletHelper,
}

impl cosmic::Application for SoullessApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.hmrdsmoke.SoullessApplet";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let helper = CosmicAppletHelper::default();
        (Self { core, helper }, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::LaunchSoulless => {
                // Spawn the Soulless launcher as a detached subprocess.
                if let Err(e) = std::process::Command::new("soulless-launcher").spawn() {
                    eprintln!("[soulless-applet] Failed to launch Soulless: {e}");
                }
                Task::none()
            }
            Message::Launched => Task::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        self.helper
            .icon_button("io.github.hmrdsmoke.Soulless")
            .on_press(Message::LaunchSoulless)
            .into()
    }

    fn view_window(&self, _id: iced::window::Id) -> Element<Self::Message> {
        // Panel applets do not have a secondary popup by default.
        widget::text("Soulless").into()
    }
}

/// Entry point called from main.rs.
pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<SoullessApplet>(true, ())
}
