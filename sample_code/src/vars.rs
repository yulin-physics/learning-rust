// Variables hold primitive data or references to data
// Variables are immutable by default, cannot reassign
// Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and next year I am {}", name, age);

    // Define constant, need to specify type and variable name usually in caps
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
