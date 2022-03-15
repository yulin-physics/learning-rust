//Interior mutability - Mutating the value inside an immutable value

// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.

// RefCell<T> is only for use in single-threaded scenarios

// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::ref_cell::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);

    println!("b after = {:?}", b);

    println!("c after = {:?}", c);
}
