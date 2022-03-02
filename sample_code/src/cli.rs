use std::env;

pub fn run() {
    // use env args from standard library
    let args: Vec<String> = env::args().collect();
    // first argument is target of the executable, and other arguments are whatever passed in
    let command = args[1].clone();
    println!("Args: {:?}", command);
    let name = "Brad";
    let status = "100%";

    if command == "hello" {
        println! {"Hi {}, how are you?", name}
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
