// Smart pointers are data structures that not only act like a pointer but also have additional metadata and capabilities.

// smart pointers implement the Deref and Drop traits

// references are pointers that only borrow data; in contrast, in many cases, smart pointers own the data they point to.

mod tree;

fn main() {
    tree::run();
}
