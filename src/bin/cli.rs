use std::env;
#[macro_use] extern crate openfocus;
use chrono::Utc;
use openfocus::error::*;
use openfocus::task::Task;
use openfocus::db::{Database, Content};
use openfocus::filter::Filter;

type MainResult = Result<(), Box<std::error::Error>>;

// converts the name of a filter to a builtin one
fn perspective_name_to_filter(name: &str) -> Filter {
    match name {
        "inbox" => Filter::new_inbox(),
        "flagged" => Filter::new_flagged(),
        "forecast" => Filter::new_forecast(),
        "projects" => Filter::new_projects(),
        "completed" => Filter::new_complete(),
        _ => panic!("unknown filter")
    }
}

// the main for filter/output mode
fn filter_main(args: Vec<String>, db: Database) -> MainResult {
    // filter the relevant tasks
    let filter = perspective_name_to_filter(&args[2]);

    // print the tasks
    for t in filter.into_iter(db.content().tasks.iter()) {
        println!("({})\t{}", t.id, t);
    }

    Ok(())
}

// the main for create mode
fn update_main(args: Vec<String>, mut db: Database) -> MainResult {
    let id = &args[3];

    // find the task
    let task: &Task = match db.content().tasks.iter().find(|t| &t.id == id) {
        Some(t) => t,
        None => return Err(err!(NotFound))
    };

    // clone the task
    let mut task = task.clone();

    // update attributes
    let mut iter = args.iter().skip(4);
    while let Some(arg) = iter.next() {
        if !arg.starts_with('-') {
            return Err(err!(InvalidArgument));
        }

        do_update(&mut task, &arg[1..], &mut iter)?;
    }

    // update the modified date attribute
    task.modified = Some(Utc::now());

    println!("{}", &task);

    // submit changes
    unimplemented!()
}

fn do_update<'a>(
    task: &mut Task,
    arg: &str,
    iter: &mut impl Iterator<Item = &'a String>
) -> MainResult {
    match arg {
        "title" | "t" => {
            task.title = match iter.next() {
                Some(s) => s.clone(),
                None => return Err(err!(InvalidArgument))
            }
        },
        "project" | "p" => {
            task.parent = Some(match iter.next() {
                Some(id) => id.clone(),
                None => return Err(err!(InvalidArgument))
            })
        },
        "note" | "n" => {
            task.note = Some(match iter.next() {
                Some(n) => n.clone(),
                None => return Err(err!(InvalidArgument))
            })
        },
        "complete" | "c" => {
            task.completed = Some(Utc::now());
        },
        "incomplete" | "ic" => {
            task.completed = None;
        },
        "flag" | "f" => {
            task.flagged = !task.flagged;
        },
        "due" | "d" => {
            task.due = Some(match iter.next() {
                Some(d) => d.parse()?,
                None => return Err(err!(InvalidArgument))
            })
        },
        "defer" => {
            task.due = Some(match iter.next() {
                Some(d) => d.parse()?,
                None => return Err(err!(InvalidArgument))
            })
        },
        "duration" | "estimate" | "e" => {
            task.estimated_duration = Some(match iter.next() {
                Some(d) => d.parse()?,
                None => return Err(err!(InvalidArgument))
            })
        }
        _ => return Err(err!(InvalidArgument))
    }

    Ok(())
}

// the main for creation mode
fn create_main(args: Vec<String>, mut db: Database) -> MainResult {
    // create the task
    let mut task = Task::default();
    task.title = args[3].clone();
    task.inbox = true;
    // write it to the database
    let delta = Content::new_task(task);
    db.write(delta)
}

// the actual main that chooses between modes
fn main() -> MainResult {
    let args: Vec<String> = env::args().collect();

    // print usage if too few arguments are passed
    if args.len() < 3 {
        println!("usage: {} [filename] [perspective / new / update]", &args[0]);
        std::process::exit(1);
    }

    // open the database
    let path = (&args[1]).into();
    let db = Database::new(path)?;

    // mode switch
    match args[2].as_ref() {
        "new" => create_main(args, db),
        "update" => update_main(args, db),
        _ => filter_main(args, db),
    }
}
