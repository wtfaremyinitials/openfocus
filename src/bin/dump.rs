use std::env;
use std::fs::File;
use openfocus::parse::parse;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {} [filename]", &args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let tasks = parse(file)?;

    for task in tasks {
        println!("{:?}", task);
    }

    Ok(())
}
