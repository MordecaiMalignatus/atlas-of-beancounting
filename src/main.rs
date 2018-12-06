#![allow(dead_code)]
extern crate chrono;
extern crate clipboard;
extern crate notify;
extern crate regex;
extern crate reqwest;
extern crate web_view;
extern crate rusqlite;
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

    let (log_sender, _log_receiver) = mpsc::channel();
    thread::spawn(move || {
        log_watcher::watch_zone_log(log_sender);
    });

    let (tooltip_sender, _tooltip_receiver) = mpsc::channel();
    thread::spawn(move || {
        tooltip_parser::spawn_tooltip_parser(clipboard_receiver, tooltip_sender);
    });

    let (_frontend_sender, frontend_receiver) = mpsc::channel();
    frontend::spawn_frontend(frontend_receiver);
}
