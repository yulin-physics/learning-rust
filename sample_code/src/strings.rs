// Primitive string = Immutable fixed-length string somewhere in memory.
// String = Growable, heap allocated data structure
pub fn run() {
    // Primitive string
    let hello = "Hello";

    // String
    let mut hi = String::from("Hello ");
    // Push char
    hi.push('W');
    // Push string
    hi.push_str("ow!");

    // Get length
    println!("{}, {}", hi.len(), hello.len());

    // Capcity in bytes
    println!("Capacity: {}", hi.capacity());
    println!("Is empty: {}", hi.is_empty());

    // Contains substring
    println!("Contains 'world' {}", hi.contains("world"));

    // Replace
    println!("Replace: {}", hi.replace("Wow", "World!"));

    // Loop through string by whitespace
    for word in hi.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);
    println!("{:?}", (hello, hi));

    // Assertion testing
    assert_eq!(2, s.len());
}
