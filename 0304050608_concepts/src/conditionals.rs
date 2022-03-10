// Conditionals - used to check  the condition and act on result

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;

    if age >= 21 && check_id {
        println!("Bartender: what would you like to drink?");
    } else if age < 21 && check_id {
        println!("Sorry you have to leave");
    } else {
        println!("Need to see your ID")
    }

    // Shorthand if, similar to ternary operator
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
