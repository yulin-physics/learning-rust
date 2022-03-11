#![allow(unused)]
// traits -  useful in the context of closures and iterators
// coherence- the orphan rule: can implement a trait on a type only if at least one of the trait or the type is local to our crate
pub trait Summary {
    // method signatures that describe the behaviors of the types that implement this trait
    fn summarize(&self) -> String {
        // Default implementations can call other methods in the same trait
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;

    // traits as parameters, syntax sugar for trait bound
    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // trait bound
    // fn notify<T:Summary>(item: &T){
    //     println!("Breaking news! {}", item.summarize());
    // }

    // multiple trait bounds
    // pub fn notify(item: &(impl Summary + Display)) {}
    // pub fn notify<T: Summary + Display>(item: &T) {}

    //trait bounds with where clause
    //     fn some_function<T, U>(t: &T, u: &U) -> i32
    //     where T: Display + Clone,
    //           U: Clone + Debug
    // {
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
