extern crate notify;

#[cfg(windows)]
extern crate clipboard_win;
#[cfg(unix)]
extern crate clipboard2;

mod clipboard_poller;
mod types;

use std::sync::mpsc;
use std::thread;

fn main() {
    println!("Hello, world!");

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        clipboard_poller::watch_clipboard(sender);
    });

    println!("{:?}", receiver.recv());
}
