// Generating f32 values between 0 and 1 from random bytes

// f32/f64
fn mock_rand_simple(n: u8) -> f32 {
    (n as f32) / 255.0
}

// shiftbyte
fn mock_rand(n: u8) -> f32 {
    // exponent of -1, underscores mark the sign/mantissa/exponent boundaries
    let base: u32 = 0b0_01111110_00000000000000000000000;
    // align the input byte to 32 bites, then increase the value by shifting its bits 15 places to the left
    let large_n = (n as u32) << 15;
    // merging base with input byte
    let f32_bits = base | large_n;
    // achieves range of 0.5-0.998
    let m = f32::from_bits(f32_bits);
    // normalise to 0.0-0.996
    2.0 * (m - 0.5)
}

pub fn run() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x77, mock_rand(0x77));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
