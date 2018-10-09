use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use types::zone_event::ZoneEvent;

pub fn watch_zone_log(s: Sender<ZoneEvent>) -> ! {
    let (watcher_sender, watcher_receiver) = channel();
    let mut watcher = raw_watcher(watcher_sender).unwrap();
    let mut log_offset = 0 as u64;
    let filepath = guess_event_path().expect("Can't start watcher without event log.");

    watcher
        .watch(filepath.clone(), RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        match watcher_receiver.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?}, {:?}, {:?}: Raw event received", path, op, cookie);
                let (last_line, new_offset) = get_last_line_of_log(filepath.as_path(), log_offset)
                    .expect("Couldn't read from file");
                log_offset = new_offset;

                println!("New last line from log: {}", last_line);
            }
            Ok(event) => println!("Got broken event: {:?}", event),
            Err(e) => println!("Caught error {:?}", e),
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

/// Get the last line of a file efficiently.  This returns an offset that is
/// intended to be fed back into the function after getting a WRITE event from
/// our watcher.
fn get_last_line_of_log(file: &Path, offset: u64) -> Result<(String, u64), Error> {
    let mut file = File::open(&file)?;
    file.seek(SeekFrom::Start(offset))?;

    let mut last_line = String::new();
    let bytes_read = file.read_to_string(&mut last_line)?;
    let new_offset = offset + bytes_read as u64;

    Ok((last_line, new_offset))
}
