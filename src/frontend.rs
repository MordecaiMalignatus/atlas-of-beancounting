use constants::IS_DEBUG;
use std::sync::mpsc::Receiver;
use types::frontend_communication::FrontendMessage;
use web_view::Content;

pub fn spawn_frontend(_receiver: Receiver<FrontendMessage>) -> () {
    let html = include_str!("../frontend/index.html");

    let view = web_view::builder()
        .title("Path of Beancounting")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(IS_DEBUG)
        .user_data(()) // data is kept in react
        .invoke_handler(|_webview, arg| {
            println!("[frontend] Caught external invokation: {}", arg);
            Ok(())
        }).build()
        .expect("Can't create front-end window");
}
