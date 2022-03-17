// type aliases
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
fn type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 8;
    println!("x + y ={}", x + y);
}

// empty type, or never type;Functions that return never are called diverging functions
// e.g. continue, panic!
fn bar()-> !{
    panic!("")
}


// dynamically sized types or unsized types
// all values of a type must use the same amount of memory
pub fn dst(){
    // let s1: str = "Hello there!";
}