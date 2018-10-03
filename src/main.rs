extern crate notify;

#[cfg(windows)]
extern crate clipboard_win;
#[cfg(unix)]
extern crate clipboard2;

mod clipboard_poller;
mod types;

fn main() {
    println!("Hello, world!");
}
