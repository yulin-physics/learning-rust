// can express only one value to compare with the patterns
// exhaustive

enum Patterns{
    Pattern
}

fn run(p: Patterns){
    match p {
        Patterns::Pattern => true,
        _ => false,
    }

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}