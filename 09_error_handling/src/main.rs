#![allow(unused)]
// Result<T, E> for recoverable errors
// panic! macro for unrecoverable error, stops execution
use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // use match expression
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // create file if doesn't exist
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("problem creating teh file: {:?}", error),
            },
            // panic for other error kinds
            other_error => {
                panic!("problem opening the file: {:?}", other_error)
            }
        },
    };

    // use closure
    let f = File::open("hello.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|e| panic!("problem creating the file: {:?}", e))
        } else {
            panic!("problem opening the file: {:?}", e)
        }
    });

    // shortcut for panic on error
    let f = File::open("hello.txt").unwrap();

    // choose panic error message with expect
    let f = File::open("hi.txt").expect("file opening error");

    Ok(())
}

// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        // read to string returns a usize, so use a underscore _ in ok to omit the usize
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// propagating errors shorcut
// ? operator: Err will be returned from the whole function as if we had used the return keyword
fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
