use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::thread::sleep;
use std::time::Duration;
use types::clipboard_event::ClipboardEvent;
#[cfg(windows)]
use clipboard_win::Clipboard;
#[cfg(unix)]
use clipboard2;

/// Poll clipboard every 200ms, trying to grab everything that happens. Might be
/// not good performance wise.
#[cfg(windows)]
pub fn watch_clipboard(s: Sender<ClipboardEvent>) -> ! {
    let mut current_content: String;
    loop {
        // TODO: Miiiiiight need better error handling. Or any at all.
        let contents = Clipboard::new().unwrap().get_string().unwrap();
        match contents {
            current_content => {},
            _ => {
                current_content = contents;
                s.send(ClipboardEvent{content: current_content});
            }
        }

        sleep(Duration::from_millis(200));
    }
}

#[cfg(unix)]
pub fn watch_clipboard(s: Sender<ClipboardEvent>) -> ! {
    panic!("Unix not yet implemented, send a patch!");
}
