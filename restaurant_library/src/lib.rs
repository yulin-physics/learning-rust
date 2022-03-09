//  crate's module tree, entire module nested inside implicit module called crate

// nest path: 
// use std::{cmp::Ordering, io};
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // construct relative paths that begin in the parent module by using super
        // parent module of back_of_house, which in this case is crate
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // all enum fields are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//  bring a path into a scope once and then call the items in that path
use crate::front_of_house::hosting;
// pub use self::front_of_house::hosting as FoH;
pub fn eat_at_restaurant_use() {
    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
