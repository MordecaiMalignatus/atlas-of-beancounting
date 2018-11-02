use chrono::prelude::*;

#[derive(Debug)]
pub enum ZoneEvent {
    ZoneChange(String, DateTime<Local>),
}
