use std::fmt;
use chrono::prelude::*;

use crate::error::*;

pub type ID = String;

fn generate_id() -> ID {
    "x1234567890".into() // TODO
}

// enumeration of the order in which subtasks can be completed
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

// a struct to represent a given task to be completed
#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    // metadata
    pub id: ID,
    pub parent: Option<ID>,
    pub rank: Option<i64>,
    pub inbox: bool,
    pub added: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
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
    pub order: Option<SubtaskOrder>,
    // TODO: repetition and clone attributes
}

// self explanatory
impl Default for Task {
    fn default() -> Self {
        Task {
            id: generate_id(),
            parent: None,
            rank: None,
            inbox: false,
            added: Utc::now(),
            modified: Some(Utc::now()),
            name: "".into(),
            note: None,
            context: None,
            flagged: false,
            due: None,
            start: None,
            completed: None,
            estimated_duration: None,
            complete_by_children: false,
            order: Some(SubtaskOrder::Sequential),
        }
    }
}

// turns a Task into a String
// examples:
//  [x] this task is complete
// ![ ] this flagged task is incomplete
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flag = if self.flagged { "!" } else { " " };
        let complete = if self.completed.is_some() { "x" } else { " " };
        let name = self.name.replace("\n", "");
        let tabs = {
            let column = 40;
            let count = std::cmp::max(column - self.name.len(), 0) / 8;
            "\t".repeat(count)
        };
        let due = if let Some(due) = self.due {
            "(".to_string() + &due.to_string() + ")"
        } else {
            "".into()
        };

        write!(
            f,
            "{}[{}] {}{}{}",
            flag,
            complete,
            name,
            tabs,
            due,
        )
    }
}
