use chrono::prelude::*;

#[derive(Debug)]
pub struct ZoneEvent {
    pub new_zone: String,
    pub timestamp: DateTime<Local>,
}
