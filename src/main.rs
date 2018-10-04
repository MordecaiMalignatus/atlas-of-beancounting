extern crate notify;
extern crate clipboard;
extern crate chrono;

mod clipboard_poller;
mod log_watcher;
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
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());

}
