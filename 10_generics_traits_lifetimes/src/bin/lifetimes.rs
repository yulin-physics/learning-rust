#![allow(unused)]
// lifetimes are a type of generic
// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

// an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // first elision rule
    fn level(&self) -> i32 {
        3
    }

    // first and third elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please; {}", announcement);
        self.part
    }
}

fn main() {
    // static lifetime - reference live fro entire duration of program
    // all string literals have staic lifetime
    let s: &'static str = "I have a static lifetime";

    let string1 = String::from("long word");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", first_sentence)
}

// add generic lifetime parameters
// lifetime annotations tell Rust how generic lifetime parameters of multiple references relate to each other
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // returned reference will be valid as long as both the parameters are
    // the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints
