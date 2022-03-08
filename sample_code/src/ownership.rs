//  OWNERSHIP RULES:
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

pub fn run() {
    a_clone();
}

// the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a move.
fn a_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);
}

// deep copy of heap data
fn a_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2)
}

// types such as integers that have a known size at compile time are stored entirely on the stack - can have COPY trait. If a type implements the Copy trait, a variable is still valid after assignment to another variable.
