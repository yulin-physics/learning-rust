use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        };
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);
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
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess).value();

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
