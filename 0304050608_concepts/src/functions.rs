pub fn run() {
    greeting("Hello", "Jane");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("{}", get_sum);

    // Closure, allows you to use outside variables
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 3))
}

// Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

// Calling a function is an expression. Calling a macro is an expression. 
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
