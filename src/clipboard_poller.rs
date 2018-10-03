use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::thread::sleep;
use types::clipboard_event::ClipboardEvent;

pub fn watch_clipboard(s: Sender<ClipboardEvent>) -> ! {
    loop {

    }
}
