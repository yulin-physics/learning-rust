// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    // The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don’t return any other value.
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);
    let (x, y, z) = (person);
    println!("{} is from {} and is {}", x, y, z);
    println!("{}", person.0.len());
}
