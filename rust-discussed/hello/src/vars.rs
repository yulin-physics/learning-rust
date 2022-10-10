pub fn run() {
    const NAME: &str = "yulin";

    let mut another_name = String::from("Mark");
    another_name.push_str(" cooper");

    let mut age = 10;

    age = 11;

    println!("name: {}, age: {}", another_name, age);
}
