// argument collecting and error handling in main.rs

// read command line arguments
use minigrep::*;
use std::{env, process};

// main function - setting up configuration or handling errors.
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        // print to standard error stream
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    println!(
        "Searching for {} in file: {}",
        config.query, config.filename
    );

    if let Err(e) = run(config) {
        // print to standard error stream
        eprintln!("Applicatin error: {}", e);
        process::exit(1);
    };
}
