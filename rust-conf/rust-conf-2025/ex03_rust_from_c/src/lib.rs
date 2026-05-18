// mangle means you can't determine what the function will be called in the binary
// not a problem in statically linking, but dynamically
#[no_mangle]
pub extern "C" fn say_hello() {
    println!("Hello world from Rust!")
}
