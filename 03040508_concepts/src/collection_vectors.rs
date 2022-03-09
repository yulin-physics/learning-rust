// Vectors - re-sizable arrays

use std::mem;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn run() {
    // new empty vector
    let v: Vec<i32> = Vec::new();
    // use vec! macro, and mut keyword to allow push
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for i in &numbers {
        println!("Number: {}", i);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    for i in &mut numbers {
        *i *= 2;
    }
    println!("Numbers Vec after mut: {:?}", numbers);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vectors occupies {} bytes", mem::size_of_val(&numbers));
    println!("{:?}", numbers);
    println!("String val: {}", numbers[0]);

    // Get slice - &[T]
    let slice: &[i32] = &numbers[2];
    println!("Slice: {:?}", slice);

    // Get value - <&T>
    match numbers.get(2) {
        some(slice) => println!("the third element is {}", slice),
        None => println!("there is no third element"),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("name")),
        SpreadsheetCell::Float(3.6),
    ];
}
