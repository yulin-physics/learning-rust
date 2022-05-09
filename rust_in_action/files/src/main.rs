//! Simulating files one step at a time.

use rand::Rng;
use std::fmt::{self, Display};

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// Represents a "file", which probably lives on a file system
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

trait Read {
    // pseudo-type Self is a placeholder for the type that will implement Read
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        // make a copy here, as .append() will shrink the input
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        // ensures sufficient space to fit the incoming data and minimizes allocations when data is inserted byte-by-byte
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        if one_in(10) {
            let err_msg = String::from("Cannot read file");
            return Err(err_msg);
        }
        Ok(read_length)
    }
}

impl File {
    /// Creates a new, empty `File`.
    /// New files are assumed to be empty, but a name is required.
    /// # Example
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    /// Returns the file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    if one_in(10) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    if one_in(10) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = Vec::new();
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    // convert Vec<u8> to String
    let text = String::from_utf8_lossy(&buffer);

    println!("{}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
