use std::env;
use std::fs::{read_dir, File};
use openfocus::db::Database;
use openfocus::filter::Filter;
use openfocus::error::*;

fn perspective_name_to_filter(name: &str) -> Filter {
    match name {
        "inbox" => Filter::new_inbox(),
        "flagged" => Filter::new_flagged(),
        "forecast" => Filter::new_forecast(),
        "projects" => Filter::new_projects(),
        "completed" => Filter::new_completed(),
        _ => panic!("unknown filter")
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // print usage if too few arguments are passed
    if args.len() < 3 {
        println!("usage: {} [filename] [perspective]", &args[0]);
        std::process::exit(1);
    }

    // open the data file
    let path = (&args[1]).into();
    let db = Database::new(path)?;
    // filter the relevant tasks
    let filter = perspective_name_to_filter(&args[2]);

    // print the tasks
    for t in filter.into_iter(db.content().tasks.iter()) {
        println!("{}", t);
    }

    Ok(())
}
