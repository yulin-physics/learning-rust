// Structs- used to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct
struct Color(u8, u8, u8);

// Unit like struct
struct AlwaysEqual;
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 

struct User {
    first_name: String,
    last_name: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    // Construct
    fn new(first: &str, last: &str) -> User {
        User {
            first_name: first.to_string(),
            last_name: last.to_string(),
            active: true,
            sign_in_count: 1,
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn run() {
    // let mut c = Color{
    //     red: 255,
    //     green:0,
    //     blue:0,
    // };

    // c.red = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = Color(255, 0, 0);
    c.0 = 200;
    println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = User::new("John", "Doe");
    let struct_update_syntax = User {
        first_name: String::from("first"),
        last_name: String::from("last"),
        ..p
    };
    p.set_last_name("Mary");
    println!(" User{}", p.full_name());
    println!(" UserTuple {:?}", p.to_tuple())
}
