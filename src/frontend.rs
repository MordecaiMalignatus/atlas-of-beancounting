use std::thread::{sleep_ms, spawn};
use web_view::{run, Content};

pub fn spawn_frontend(_receiver: Receiver<FrontendMessage>) -> () {
    let html = include_str!("../frontend/index.html");
    let content = Content::Html(html);
    let resizeable = true;
    let size = (800, 600);
    let debug = true;

    run(
        "Test window!",
        content,
        Some(size),
        resizeable,
        debug,
        move |_webview| {},
        move |_webview, _event, _user_data| {},
        (),
    );
}
