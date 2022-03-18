fn main() {
    let mut i1 = 10;
    println!("i1 address {:p}, i1 value {}", &i1, i1);
    let i2 = 20;
    // not allocating memory here, just naming existing memory
    let i3 = &mut i1;
    *i3 = 30;
    println!("i2 address {:p}, i2 value {}", &i2, i2);
    println!("i3 address {:p}, i3 value {}", i3, *i3);
    println!("i1 address {:p}, i1 value {}", &i1, i1);
}
