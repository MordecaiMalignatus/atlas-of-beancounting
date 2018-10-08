use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender};
use std::time::Duration;
use std::path::PathBuf;
use std::io::ErrorKind;
use std::io::Error;
use types::zone_event::ZoneEvent;
use notify::{Watcher, raw_watcher, RecursiveMode, RawEvent};

pub fn watch_zone_log(s: Sender<ZoneEvent>) -> ! {
    let  (watcher_sender, watcher_receiver) = channel();
    let mut watcher = raw_watcher(watcher_sender).unwrap();
    let filepath = guess_event_path().expect("Can't start watcher without event log.");

    watcher.watch(filepath, RecursiveMode::NonRecursive).unwrap();

    loop {
        match watcher_receiver.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) =>
                println!("{:?}, {:?}, {:?}: Raw event received", path, op, cookie),
            Ok(event) =>
                println!("Got broken event: {:?}", event),
            Err(e) =>
                panic!("Caught error {:?}", e),
        }
    }
}

/// We don't actually know where on the system the log file is, so we're gonna
/// take some educated guesses and give up if we're wrong.
/// 1. Steam path on windows.
/// 2. Standalone launcher on windows.
/// If it's not there, I'm not sure where to find it, pull requests/issues appreciated.
fn guess_event_path() -> Result<PathBuf, Error> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}
