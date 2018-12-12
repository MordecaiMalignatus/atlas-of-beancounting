//! This is the module for the database actor. You can interact with it by
//! sending it messages, usually a request to persist something, and looking for
//! the return value.
use std::sync::mpsc::{Receiver, Sender};
use types::database::DatabaseMessage;

pub struct DatabaseBot {
    pub receiver: Receiver<DatabaseMessage>,
    pub sender: Sender<DatabaseMessage>,
    // connections, other config things.
}

impl DatabaseBot {
    pub fn spawn(
        receiver: Receiver<DatabaseMessage>,
        sender: Sender<DatabaseMessage>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            let databaseBot = DatabaseBot { receiver, sender };
            databaseBot.run()
        })
    }

    pub fn run() -> () {
        unimplemented!()
    }
}
