/// cargo run -- "pattern" "filename"
/// cargo run -- picture readme.md
///
// look outside of the standard library for this crate
extern crate clap;
extern crate regex;
// bring into local scope
use clap::{App, Arg};
use regex::Regex;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(
    reader: &mut T,
    search_term: Regex,
    context_lines: usize,
    cache_reader: T,
) {
    // hold line numbers of matches
    let mut tags: Vec<usize> = Vec::new();
    // hold context lines for each match
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    for (idx, line_) in reader.by_ref().lines().enumerate().peekable() {
        let line = line_.unwrap();
        match search_term.find(&line) {
            Some(_) => {
                tags.push(idx);
                let v = Vec::with_capacity(1 + 2 * context_lines);
                ctx.push(v);
            }
            None => (),
        }
    }

    if tags.len() == 0 {
        return;
    }

    for (i, line_) in cache_reader.lines().enumerate() {
        let line = line_.unwrap();
        for (j, tag) in tags.iter().enumerate() {
            // subtraction that returns 0 on integer underflow
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let local_ctx = (i, line.clone());
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

fn process_stdin<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let input = args.value_of("input").unwrap_or("-");

    let pattern = args.value_of("pattern").unwrap();
    let context_lines = 2;
    let search_term = Regex::new(pattern).unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_stdin(reader, search_term);
    } else {
        let f = File::open(input).unwrap();
        let mut reader = BufReader::new(&f);
        let f = File::open(input).unwrap();
        let cache_reader = BufReader::new(&f);
        process_lines(&mut reader, search_term, context_lines, cache_reader);
    }

    // let mut line = String::new();
    // loop {
    //     let len = reader.read_line(&mut line).unwrap();
    //     if len == 0 {
    //         break;
    //     }

    //     println!("{} ({} bytes long)", line, len);

    //     // shrink String back to length 0, preventing lines form persisting into the following one
    //     line.truncate(0);
    // }
}
