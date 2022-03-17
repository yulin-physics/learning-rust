// function pointers
// Unlike closures, fn is a type rather than a trait
// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce)
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

pub fn func_pointers() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);


    // closure
    let list_of_num = vec![1, 2, 3, 4, 5];
    let list_of_strings:Vec<String> = list_of_num.iter().map(|i| i.to_string()).collect();
    // function
    let list_of_strings:Vec<String> = list_of_strings.iter().map(ToString::to_string).collect();
    // use () as initializer syntax for function pointers
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("Status: {:?}", list_of_statuses)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}