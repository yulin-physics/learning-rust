#![allow(unused)]
// use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

// Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// trait bound to conditionally impl methods
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x: {}", self.x);
        } else {
            println!("The largest number is y: {}", self.y);
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

// declare T just after impl to bring generics into scope
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// method on concrete type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("Hello, world!");
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    // types like i32 and char that have a known size can be stored on the stack, so they implement the Copy trait.
    let mut largest = list[0];

    for &item in list {
        //  > operator is defined as a default method on the standard library trait std::cmp::PartialOrd
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // pattern matching and destructuring each &i32
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
