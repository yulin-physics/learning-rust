// specify linkage - default and most common is C, but there is one for wasm
// extern "C" {
//     fn say_hello();
// }

// Import the auto-generated header
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    unsafe  {
        say_hello();
    }
}
