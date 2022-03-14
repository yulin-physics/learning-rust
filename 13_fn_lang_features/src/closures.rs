// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.

// All closures:
// - implement at least one of the traits: Fn, FnMut, or FnOnce.
// - capture their environment and access variables from the scope in which they’re defined.

// FnOnce consumes the captured variables by take ownership of these variables and move them into the closure when it is defined.
// FnMut can change the environment because it mutably borrows values.
// Fn borrows values from the environment immutably.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// Fn trait bound on T generics defines a closure
struct Casher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Casher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Casher<T> {
        Casher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                // need surround closure with parentheses to call
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    capture_env();
}

fn capture_env() {
    let x = 4;
    // implements Fn
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));

    let u = vec![1, 2, 3];
    // implements FnOnce
    let equal_to_u = move |z| z == u;
    // println!("can't move x here: {:?}", u);
    let v = vec![1, 2, 3];
    assert!(equal_to_u(v));
}

fn generate_workout(intensity: u32, random_number: u32) {
    // expensive_closure contains the definition of an anonymous function, not the resulting value of calling the anonymous function
    let mut expensive_closure = Casher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Casher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
