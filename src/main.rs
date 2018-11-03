extern crate chrono;
extern crate clipboard;
extern crate notify;
extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate hyper;

mod clipboard_poller;
mod log_watcher;
mod tooltip_parser;
mod types;

use std::sync::mpsc;
use std::thread;

fn main() {
    println!("Hello, world!");

    let (clipboard_sender, clipboard_receiver) = mpsc::channel();
    thread::spawn(move || {
        clipboard_poller::watch_clipboard(clipboard_sender);
    });

    let (log_sender, log_receiver) = mpsc::channel();
    thread::spawn(move || {
        log_watcher::watch_zone_log(log_sender);
    });

    println!("{:?}", clipboard_receiver.recv());
    println!("{:?}", log_receiver.recv());
}
