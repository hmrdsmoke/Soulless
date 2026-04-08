// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use iced::{widget::container, Element, Length, Theme, Task, Command};
// Removed Size, Point, window, Application due to iced 0.14 removing them entirely :: MRV 
use std::fs::OpenOptions;
use fs2::FileExt; // for single-instance lock

mod search;
mod drawers;
mod position;

use search::Message as SearchMessage;
use position::DockPosition;

pub enum Message {
// removed # from start of line ::MRV
    Search(SearchMessage),
    Close,           // click outside or Esc
}

fn main() -> iced::Result {
    // Single-instance check (real launcher feel)
    if !ensure_single_instance() {
        eprintln!("Soulless is already running — bringing existing instance forward.");
        // TODO: send signal via Unix socket to show window (next step)
        return Ok(());
    }

    Soulless::run(iced::Settings {
        window: window::Settings {
            size: DockPosition::BottomLeft.window_size(), // default starting position
            position: window::Position::Specific(DockPosition::BottomLeft.window_position()),
            decorations: false,          // borderless dock-like
            transparent: false,          // set true later for fancy blur
            always_on_top: true,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Soulless {
    search: search::Search,
    position: DockPosition,
}

impl Application for Soulless {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Message>) {
        (Self {
            search: search::Search::new(),
            position: DockPosition::BottomLeft, // TODO: load from config
        }, Task::none())
    }

    fn title(&self) -> String {
        "Soulless".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Search(msg) => {
                if let Some(exec) = self.search.update(msg) {
                    let _ = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&exec)
                        .spawn();
                    // Auto-close after launch (classic launcher behavior)
                    return iced::exit();
                }
                Task::none()
            }
            Message::Close => iced::exit(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = drawers::view(&self.search)
            .map(Message::Search);

        // Click-outside-to-close container (expand later with mouse events)
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .on_press(Message::Close) // background click closes
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

/// Simple single-instance using file lock + XDG path.
/// Keeps binary small and startup fast. 
/// MRV: This gives the "real launcher" feel — second launch just activates the first.
fn ensure_single_instance() -> bool {
    let lock_path = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("soulless/soulless.lock");

    std::fs::create_dir_all(lock_path.parent().unwrap()).ok();

    if let Ok(file) = OpenOptions::new().write(true).create(true).open(&lock_path) {
        if file.try_lock_exclusive().is_ok() {
            return true; // we are the only instance
        }
    }
    false
}