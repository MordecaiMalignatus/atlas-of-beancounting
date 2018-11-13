extern crate chrono;
extern crate clipboard;
extern crate notify;
extern crate regex;
extern crate reqwest;
extern crate web_view;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

mod clipboard_poller;
mod log_watcher;
mod tooltip_parser;
mod types;
mod web_client;
mod frontend;
mod constants;

use std::sync::mpsc;
use std::thread;

fn main() {
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

    frontend::spawn_frontend();
}
