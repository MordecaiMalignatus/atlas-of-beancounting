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

    pub fn run(&mut self) -> () {
        unimplemented!();
        loop {
            match self.receiver.recv() {
                Ok(o) => match o {
                    _ => unimplemented!()
                }
                Err(e) => {
                    println!("[DatabaseBot] Can't read from input channel, exiting");
                    self.sender.send(DatabaseMessage::Panic{reason: String::from("[DatabaseBot] Can't read from input channel, exiting.")});
                    painic!("Can't read from input channel")
                }
            }
        }
    }
}
