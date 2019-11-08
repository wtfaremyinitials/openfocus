use std::env;
use std::fs::{read_dir, File};
use openfocus::parse::parse;
use openfocus::filter::Filter;
use openfocus::error::*;

fn open_file(path: &str) -> Result<File, Error> {
    let data_path = read_dir(path)?
        .filter(|p| {
            p.is_ok() && p.as_ref().unwrap()
                .path().to_str().unwrap()
                .contains("00000000000000")
        })
        .next()
        .expect("error reading file")
        .expect("couldn't find data file")
        .path();

    Ok(File::open(data_path)?)
}

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

    if args.len() < 3 {
        println!("usage: {} [filename] [perspective]", &args[0]);
        std::process::exit(1);
    }

    let file = open_file(&args[1]);
    let tasks = parse(file?)?;
    let filter = perspective_name_to_filter(&args[2]);

    for t in filter.into_iter(tasks.iter()) {
        println!("{}", t);
    }

    Ok(())
}
