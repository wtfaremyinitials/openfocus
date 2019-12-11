use chrono::prelude::*;

use crate::util::ID;

// struct to represent a perspective. WORK IN PROGRESS
#[derive(Debug, PartialEq, Eq)]
pub struct Perspective {
    // metadata
    pub id: ID,
    pub added: DateTime<Utc>
    // attributes
}
