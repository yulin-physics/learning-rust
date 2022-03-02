// Public function
pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatiing
    println!("{} is from {}", "rice", "field");

    // Positional arguments
    println!("{0} is from {1} and {0} is {2}", "rice", "field", "yum");

    // Named arguments
    println!("{food} is from {place}", food = "rice", place = "field");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // String literal for numbers
    println!("Number: {}", 1);

    // Basic math
    println!("10+10={}", 10 + 10);
}
