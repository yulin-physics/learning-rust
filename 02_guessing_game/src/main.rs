use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Bind new instance of String to variable guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            // crash this program when a problem occurs
            .expect("Failed to read line");

        // reuse guess variable: shadow the previous value of guess with a new one.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // variant of the Ordering enum, match checking each arm’s pattern
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
