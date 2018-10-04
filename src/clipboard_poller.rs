use std::sync::mpsc::{Sender};
use std::thread::sleep;
use std::time::Duration;
use types::clipboard_event::ClipboardEvent;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

/// Poll clipboard every 200ms, trying to grab everything that happens. Might be
/// not good performance wise.
pub fn watch_clipboard(s: Sender<ClipboardEvent>) -> ! {
    let mut current_content = String::new();
    loop {
        // TODO: Miiiiiight need better error handling. Or any at all.
        let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
        let contents = context.get_contents().unwrap();
        match contents {
            _ if current_content == contents => {},
            _ => {
                current_content = contents;
                s.send(ClipboardEvent{content: current_content.clone()}).unwrap();
            }
        }

        sleep(Duration::from_millis(200));
    }
}
