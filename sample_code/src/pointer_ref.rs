// Reference pointers - point to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    /*   With non primitives, if you assign another variable to a piece of data,
    the first variable will no longer hold that value

    non-primitives **move** easily, primitives **copy** easily

    Can't point directly if it is not a primitive
    You will need to use a reference (&) to point to the resource for non primitives, so called **borrow**
    */

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("values : {:?}", (&vec1, vec2));


    let mut s = String::from("hello");

    mutable_ref(&mut s);

    println!("{}", s);
}


// Mutable borrow - can have only one mutable reference to a particular piece of data at a time

// can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.
fn mutable_ref(some_string: &mut String) {
    some_string.push_str(", world!")
}
