// Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use; only for use in single-threaded scenarios

// weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.

/// multiple owners, immutable

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::ref_counting::List::{Cons, Nil};
use std::rc::Rc;

pub fn run() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // clone the Rc<List> that a is holding
    // The call to Rc::clone only increments the reference count, not a deep copy
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
