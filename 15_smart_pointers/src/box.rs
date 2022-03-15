// Using Box<T> (stored on the stack) to Point to Data on the Heap

// recursive type e.g. cons list, constructs a new pair from its two arguments, which usually are a single value and another pair.

// List type doesn’t have a known size

// single owner, mutable
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::Box::List::{Cons, Nil};
use std::ops::Deref;

pub fn run() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // implement deref trait
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // deref coercion
    let m = MyBox::new(String::from("rust"));
    hello(&m);
    hello(&(*m)[..]);

    // implement drop trait
    // Variables are dropped in the reverse order of their creation
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // drop a value early i.e. before end of scope
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.")
}

// The MyBox type is a tuple struct with one element of type T
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // Without the Deref trait, the compiler can only dereference & references.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// deref coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

// Specify the code to run when a value goes out of scope by implementing the Drop trait
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
