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

fn destructuring() {
    // destructuring structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    fn main() {
        let p = Point { x: 0, y: 7 };
    
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    
    // destructuring enums
}

struct Point {
    x: i32,
    y: i32,
}
