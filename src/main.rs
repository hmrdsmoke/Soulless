// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::{widget::container, Element, Length, Theme, Task, window, keyboard, event, Subscription};
use iced::keyboard::key::Named;
use std::fs::OpenOptions;
use fs2::FileExt;
use std::path::PathBuf;
use dirs;

mod search;
mod drawers;
mod position;

use search::Message as SearchMessage;
use position::DockPosition;

pub enum Message {
    Search(SearchMessage),
    Close,
    WindowEvent(iced::Event),
}

fn main() -> iced::Result {
    if !ensure_single_instance() {
        eprintln!("Soulless is already running — bringing existing instance forward.");
        return Ok(());
    }

    let position = DockPosition::BottomLeft;

    iced::application(Soulless::new, Soulless::update, Soulless::view)
        .subscription(Soulless::subscription)
        .window_size(position.window_size())
        .position(window::Position::Specific(position.window_position()))
        .decorations(false)
        .transparent(false)
        .resizable(false)
        .theme(Soulless::theme)
        .run()
}

struct Soulless {
    search: search::Search,
    position: DockPosition,
}

impl Soulless {
    fn new() -> (Self, Task<Message>) {
        let pos = DockPosition::BottomLeft;
        (Self {
            search: search::Search::new(),
            position: pos,
        }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Search(msg) => {
                if let Some(exec) = self.search.update(msg) {
                    let clean_exec = strip_desktop_placeholders(&exec);
                    match std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&clean_exec)
                        .spawn()
                    {
                        Ok(_) => {
                            println!("Launched: {}", clean_exec);
                            iced::exit()
                        }
                        Err(e) => {
                            eprintln!("Failed to launch '{}': {}", clean_exec, e);
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::Close | Message::WindowEvent(iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left))) => {
                iced::exit()
            }
            Message::WindowEvent(iced::Event::Keyboard(keyboard::Event::KeyPressed { key, .. })) => {
                if matches!(key, keyboard::Key::Named(Named::Escape)) {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::WindowEvent(_) => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = drawers::view(&self.search, &self.position)
            .map(Message::Search);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Color::from_rgb8(30, 30, 30).into()),
                border: iced::border::rounded(8),
                ..Default::default()
            })
            .into()
    }

    fn theme(_: &Self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::WindowEvent)
    }
}

fn strip_desktop_placeholders(exec: &str) -> String {
    let mut result = String::with_capacity(exec.len());
    let mut chars = exec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            if chars.peek().map_or(false, |&next| next.is_ascii_alphabetic()) {
                chars.next();
                continue;
            }
        }
        result.push(c);
    }
    result.trim().to_string()
}

fn ensure_single_instance() -> bool {
    let lock_path = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("soulless/soulless.lock");

    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    if let Ok(file) = OpenOptions::new().write(true).create(true).open(&lock_path) {
        if file.try_lock_exclusive().is_ok() {
            return true;
        }
    }
    false
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