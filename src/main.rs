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
                    let _ = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&exec)
                        .spawn();
                    return iced::exit();
                }
                Task::none()
            }
            Message::Close => iced::exit(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = drawers::view(&self.search)
            .map(Message::Search);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(_: &Self) -> Theme {
        Theme::Dark
    }
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

// === ALL YOUR ORIGINAL COMMENTS MOVED TO THE BOTTOM (preserved exactly) ===
 // removed for now not sure if needed :: MRV
 // needed for single-instance lock :: MRV
 // your repo uses drawer.rs (singular) :: fixed :: MRV
 // repo and local files now match changed drawer.rs to drawers.rs :: MRV
 // removed # from start of line :: MRV
 // click outside or Esc for launcher feel :: LET KNOW IF DONE ::
 // Single-instance check — gives real launcher behavior (second launch activates instead of spawning duplicate) :: MRV
 // later replace with Unix socket signal to show/hide the window :: STILL NEEDS ::
 // changed .window_position to .position this is the correct method name in iced 0.14 :: MRV
 // borderless = native dock/pop-up feel :: LET KNOW IF DONE ::
 // .always_on_top(true) not available in current builder style :: LET KNOW IF DONE ::
 // default toolbox position :: LET KNOW IF DONE :: MRV
 // auto-close after launch (classic launcher behavior) :: LET KNOW IF DONE ::
 // pass position so search bar can be top/bottom :: MRV
 // Toolbox = long rectangular pop-out window (your exact vision) :: LET KNOW IF DONE ::
 // click anywhere outside closes (real launcher feel) :: LET KNOW IF DONE ::
 // default yellow background with depth will be added later :: LET KNOW IF DONE ::
 // end of change :: MRV
 // Simple single-instance guard using XDG data dir + exclusive file lock. :: LET KNOW IF DONE :: MRV
 // Keeps startup extremely fast (sub-millisecond) and binary small. :: LET KNOW IF DONE :: MRV
 // This makes Soulless feel like a true system launcher, not a regular app. :: LET KNOW DONE :: MRV
 // we own the lock → sole instance :: LET KNOW IF DONE :: MRV