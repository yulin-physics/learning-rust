// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:

// Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// Attribute-like macros that define custom attributes usable on any item
// Function-like macros that look like function calls but operate on the tokens specified as their argument

//a macro that can take any number of arguments of any type and can generate code

pub fn declarative_macros() {
    let v: Vec<u32> = vec![1, 2, 3];
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

//  three kinds of procedural macros (custom derive, attribute-like, and function-like)
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

pub fn procedural_macros() {
    Pancakes::hello_macro();
}
