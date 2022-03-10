// Enums allow you to define a type by enumerating its possible variants.
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

// the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,

        // pattern bind to value state, get inner state value out of the Coin enum variant for Quarter.
        Coin::Quarter(state) => {
            println!("states quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(v: Option<i32>) -> Option<i32> {
    match v {
        None => None,
        Some(v) => Some(v + 1),
    }
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        // variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}
pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hi"));
    m.call();

    let option_number: Option<i32> = None;

    let coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five {:?}, six {:?}, none {:?}", five, six, none);

    ////////////////////////////////////////////if let//////////////
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {:?}", max);
    }
}
