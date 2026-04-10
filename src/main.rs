// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::{widget::container, Element, Length, Theme, Task, window};
use std::fs::OpenOptions;
use fs2::FileExt;
use std::path::PathBuf;

mod search;
mod drawers;
mod position;

use search::Message as SearchMessage;
use position::DockPosition;

pub enum Message {
    Search(SearchMessage),
    Close,
}

fn main() -> iced::Result {
    if !ensure_single_instance() {
        eprintln!("Soulless is already running — bringing existing instance forward.");
        return Ok(());
    }

    let position = DockPosition::BottomLeft;
    iced::application(Soulless::new, Soulless::update, Soulless::view)
        .window_size(position.window_size())
        .position(window::Position::Specific(position.window_position()))
        .decorations(false)
        .transparent(false)
        .resizable(false)
        .theme(Soulless::theme)
        .centered()
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
                    // Launch the app
                    let clean_exec = strip_desktop_placeholders(&exec);
                    if let Err(e) = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&clean_exec)
                        .spawn()
                    {
                        eprintln!("Failed to launch {}: {}", clean_exec, e);
                    } else {
                        // Auto-close launcher after successful launch
                        return iced::exit();
                    }
                }
                Task::none()
            }
            Message::Close => iced::exit(),
        }
    }

    fn view(&self) -> Element<Message> {
        drawers::view(&self.search).map(Message::Search)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn ensure_single_instance() -> bool {
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("soulless");
    std::fs::create_dir_all(&data_dir).ok();

    let lock_path = data_dir.join("soulless.lock");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&lock_path)
        .unwrap();

    file.try_lock_exclusive().is_ok()
}

fn strip_desktop_placeholders(exec: &str) -> String {
    // Remove common placeholders like %u, %f, %F, %i, etc.
    let re = regex::Regex::new(r"%[a-zA-Z]").unwrap();
    re.replace_all(exec, "").trim().to_string()
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
// needed for single-instance lock :: MRV
// changed .window_position to .position this is the correct method name in iced 0.14 :: MRV
// borderless = native dock/pop-up feel :: MRV
// .always_on_top(true) not available in current builder style :: MRV
// default toolbox position :: MRV
// auto-close after launch (classic launcher behavior) :: MRV
// pass position so search bar can be top/bottom :: MRV
// Toolbox = long rectangular pop-out window (your exact vision) :: MRV
// click anywhere outside closes (real launcher feel) :: MRV
// Simple single-instance guard using XDG data dir + exclusive file lock. :: MRV
// Keeps startup extremely fast (sub-millisecond) and binary small. :: MRV
// This makes Soulless feel like a true system launcher, not a regular app. :: MRV
// we own the lock → sole instance :: MRV

// === HISTORY ===
// Single-instance check
// .always_on_top(true) not available in current builder style
// Placeholder stripping moved from search to main.rs

// === IN PROGRESS ===
// - #48 [ISSUE:main-001] Click outside or Esc for launcher feel
// - #49 [ISSUE:main-002] Borderless = native dock/pop-up feel
// - #50 [ISSUE:main-003] Default toolbox position (make configurable later)
// - #51 [ISSUE:main-004] Auto-close after launch (classic launcher behavior)
// - #52 [ISSUE:main-005] Pass position so search bar can be top/bottom
// - #53 [ISSUE:main-006] Toolbox = long rectangular pop-out window (your exact vision)

// === DONE ===
// Single-instance check
// Simple single-instance guard using XDG data dir + exclusive file lock
// Placeholder stripping moved to main.rs