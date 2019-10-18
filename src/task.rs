use chrono::prelude::*;

use crate::error::*;

pub type ID = String;

#[derive(Debug)]
pub enum SubtaskOrder {
    Parallel,
    Sequential,
}

impl std::str::FromStr for SubtaskOrder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parallel" => Ok(SubtaskOrder::Parallel),
            "sequential" => Ok(SubtaskOrder::Sequential),
            _ => Err(Box::new(OpenFocusError::Parse)),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    // metadata
    pub id: ID,
    pub parent: Option<ID>,
    pub rank: i64,
    pub inbox: bool,
    pub added: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    // attributes
    pub name: String,
    pub note: Option<String>,
    pub context: Option<ID>,
    pub flagged: bool,
    pub estimated_duration: u64,
    pub complete_by_children: bool,
    pub order: SubtaskOrder,
    // TODO: repetition and clone attributes
}
