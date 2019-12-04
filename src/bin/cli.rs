use std::env;
use openfocus::task::Task;
use openfocus::db::{Database, Content};
use openfocus::filter::Filter;

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
fn filter_main(args: Vec<String>, db: Database) -> Result<(), Box<std::error::Error>> {
    // filter the relevant tasks
    let filter = perspective_name_to_filter(&args[2]);

    // print the tasks
    for t in filter.into_iter(db.content().tasks.iter()) {
        println!("{}", t);
    }

    Ok(())
}

// the main for creation mode
fn create_main(args: Vec<String>, mut db: Database) -> Result<(), Box<std::error::Error>> {
    // create the task
    let mut task = Task::default();
    task.name = args[3].clone();
    task.inbox = true;
    // write it to the database
    let delta = Content::new_task(task);
    db.write(delta)
}

// the actual main that chooses between modes
fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // print usage if too few arguments are passed
    if args.len() < 3 {
        println!("usage: {} [filename] [perspective / new]", &args[0]);
        std::process::exit(1);
    }

    // open the database
    let path = (&args[1]).into();
    let db = Database::new(path)?;

    // mode switch
    if &args[2] == "new" {
        create_main(args, db)
    } else {
        filter_main(args, db)
    }
}
