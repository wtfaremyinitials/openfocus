use chrono::prelude::*;

pub type ID = String;

fn generate_id() -> ID {
    "x1234567890".into() // TODO
}

// struct to represent a perspective. WORK IN PROGRESS
#[derive(Debug, PartialEq, Eq)]
pub struct Perspective {
    // metadata
    pub id: ID,
    pub added: DateTime<Utc>
    // attributes
}
