use std::fmt;
use chrono::prelude::*;
use colored::*;

use crate::util::{ID, generate_id};
use crate::error::*;

// enumeration of the order in which subtasks can be completed
#[derive(Debug, PartialEq, Eq, Clone)]
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
            _ => Err(crate::err!(Parse)),
        }
    }
}

// a struct to represent a given task to be completed
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    // metadata
    pub id: ID,
    pub parent: Option<ID>,
    pub rank: Option<i64>,
    pub inbox: bool,
    pub added: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
    // attributes
    pub title: String,
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
            title: "".into(),
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
        let flag = if self.flagged { "!" } else { " " }.bold();
        let complete = if self.completed.is_some() { "x" } else { " " };
        let mut title = self.title.replace("\n", "");
        if self.flagged { title = title.bold().to_string() }
        let tabs = {
            let column = 40;
            let count = std::cmp::max(column - self.title.len(), 0) / 8;
            "\t".repeat(count)
        };
        let due = if let Some(due) = self.due {
            let date_str = "(".to_string() + &due.to_string() + ")";
            if due < Utc::now() && !self.completed.is_some() {
                date_str.red().to_string()
            } else {
                date_str
            }
        } else {
            "".into()
        };

        let mut out = format!(
            "{}[{}] {}{}{}",
            flag,
            complete,
            title,
            tabs,
            due,
        );

        if self.completed.is_some() {
            out = out.strikethrough().to_string();
        }

        write!(f, "{}", out)
    }
}
