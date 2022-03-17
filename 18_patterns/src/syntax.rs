fn named_variables() {
    let x = Some(5);
    let y = 10;

    // new scope, shadows existing variables
    match x {
        Some(50) => println!("Got 50"),
        // this is a new y variable, not the y we declared at the beginning with the value 10
        Some(y) => println!("Matched, y= {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn ranges_of_val() {
    let x = 5;

    match x {
        1..=5 => println!("one through five inclusive"),
        _ => println!("anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen
pub fn match_guards() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //use match guards to solve our pattern-shadowing problem
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // y is the y in outer scope, NOT a new variable; n is a new variable
        // n==y is not a pattern, so no new variables introduced
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end, x = {:?}, y = {}", x, y);
}

// The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches a pattern.
pub fn bindings() {
    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello {
            // id:3..=7
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
        // shorthand syntax
        HelloMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

enum HelloMessage {
    Hello { id: i32 },
}

pub fn ignoring_val() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);

    // _ vs _variable
    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // ignore remaining parts
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

pub fn destructuring() {
    // destructuring structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // destructuring enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in x {} and in y {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
