use std::fmt;
use chrono::prelude::*;

use crate::error::*;

pub type ID = String;

fn generate_id() -> ID {
    "x1234567890".into() // TODO
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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
    pub due: Option<DateTime<Utc>>,
    pub start: Option<DateTime<Utc>>,
    pub completed: Option<DateTime<Utc>>,
    pub estimated_duration: Option<u64>,
    pub complete_by_children: bool,
    pub order: SubtaskOrder,
    // TODO: repetition and clone attributes
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: generate_id(),
            parent: None,
            rank: 0,
            inbox: false,
            added: Utc::now(),
            modified: Utc::now(),
            name: "".into(),
            note: None,
            context: None,
            flagged: false,
            due: None,
            start: None,
            completed: None,
            estimated_duration: None,
            complete_by_children: false,
            order: SubtaskOrder::Sequential,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flag = if self.flagged { "!" } else { " " };
        let complete = if self.completed.is_some() { "x" } else { " " };
        let name = self.name.replace("\n", "");
        let due = if let Some(due) = self.due {
            "(".to_string() + &due.to_string() + ")"
        } else {
            "".into()
        };

        write!(
            f,
            "{}[{}] {}\t{}",
            flag,
            complete,
            name,
            due,
        )
    }
}
