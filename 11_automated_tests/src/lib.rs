// Unit tests

// assert_eq! and assert_ne! macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits. For structs and enums that you define,
// -  implement PartialEq to assert that values of those types are equal or not equal.
//  - implement Debug to print the values when the assertion fails.
#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            )
        }
        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    println!("print me");
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    // tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this tests module.
    use super::*;

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run...
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    // #[should_panic]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn greeting_contains_name() {
        // custom failure messages
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn it_adds_two_and_two() {
        // expected and actual; order of arguments does not matter
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
