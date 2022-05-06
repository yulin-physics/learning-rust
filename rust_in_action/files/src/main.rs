#![allow(unused_variables)]

// alias for String, inherits of all String methods
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// relax compiler warning about an unused function
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    // read(f1, vec![]);
    close(&mut f1);
}
