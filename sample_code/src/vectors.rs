// Vectors - re-sizable arrays

use std::mem;

pub fn run() {
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

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec after mut: {:?}", numbers);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vectors occupies {} bytes", mem::size_of_val(&numbers));
    println!("{:?}", numbers);
    println!("String val: {}", numbers[0]);

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice)
}
