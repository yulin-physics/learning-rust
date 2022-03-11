// read command line arguments
use minigrep::*;
use std::{env, process};

// main function - setting up configuration or handling errors.
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    println!(
        "Searching for {} in file: {}",
        config.query, config.filename
    );

    if let Err(e) = run(config) {
        println!("Applicatin error: {}", e);
        process::exit(1);
    };
}
