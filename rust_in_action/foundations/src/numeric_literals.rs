pub fn run() {
    // base 2
    let three = 0b11;
    // base 8
    let thirty = 0o36;
    // base 16
    let three_hundred = 0x12c;

    // isize,usize–integersthatassumeCPU’s"native"width(e.g.in64-bit CPUs, usize and isize will be 64 bits wide)

    println!("{} {} {}", three, thirty, three_hundred);
    println!("{:b} {:b} {:b}", three, thirty, three_hundred);
    println!("{:o} {:o} {:o}", three, thirty, three_hundred);
    println!("{:x} {:x} {:x}", three, thirty, three_hundred);
}
