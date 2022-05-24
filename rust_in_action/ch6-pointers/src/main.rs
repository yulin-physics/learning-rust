use std::mem::size_of;
use std::borrow::Cow; // copy on write
use std::ffi::CStr; // 0-terminated strings
use std::os::raw::c_char; // type alias for i8

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    // integer
    let a: usize = 42;
    // smart pointer
    let b: &[u8; 10] = &B;
    // raw pointer
    let c: Box<[u8]> = Box::new(C);
    println!("size {:?} bytes", size_of::<&[u8; 10]>());

    // String is smart pointer type
    let b:String; 
    let c:Cow<str>;

    unsafe {
        // references cannot be cast directly to *mut T
        let b_ptr= &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("a : {}, b : {}, c : {}", a, b, c);
}
