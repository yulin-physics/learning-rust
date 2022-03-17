// unsafe superpowers
use std::slice;

// global variables are called static variables in Rust; SCREAMING_SNAKE_CASE
// static variables have a fixed memory address and can be mutable
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn global_var() {
    add_to_count(3);
    unsafe {
        println!("counter is: {}", COUNTER);
    }
}

pub fn deref_raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2)
    }
}

// Within the extern "C" block, we list the names and signatures of external functions from another language we want to call. The "C" part defines which application binary interface (ABI) the external function uses
extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn foreign_func_interface() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!")
}

pub fn safe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    //  borrowing from the same slice twice
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // get raw pointer of a slice, *mut i32
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            // get raw pointer starts at mid
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn unsafe_functions() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}
