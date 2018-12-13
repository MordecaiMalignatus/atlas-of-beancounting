use types::item::Item;

/// Communicate with the database actor.
pub enum DatabaseMessage {
    Save(Item),
    ShutDown,
    Panic{reason: String},
}
