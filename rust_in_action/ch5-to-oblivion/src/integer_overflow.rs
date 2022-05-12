use std::mem;

pub fn run() {
    let zero: u16 = 0b0000_0000_0000_0000;
    let one: u16 = 0b0000_0000_0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;

    let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}, ..., ", zero, one, two);
    println!(
        "{}, {}, {}",
        sixtyfivethousand_533, sixtyfivethousand_534, sixtyfivethousand_535
    );

    let (a, b) = (200, 200);

    let c: u8 = a + b;
    println!("200 + 200 = {}", c);

    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];

    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let (a, b): (i32, i32) = unsafe {
        // treat u8 bytes as i32 type
        (mem::transmute(big_endian), mem::transmute(little_endian))
    };

    println!("{} vs {}", a, b);
}
