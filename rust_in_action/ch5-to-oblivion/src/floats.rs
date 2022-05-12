const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

pub fn run() {
    let n: f32 = 42.42;

    let (signbit, exponent, fraction) = deconstruct_f32(n);
    // let (sign, exponent, mantissa) = decode_f32_parts(signbit, exponent, mantissa);
}

fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
    let n_: u32 = unsafe { std::mem::transmute(n) };

    // strip 31 unwanted bits and leave only sign bit
    let sign = (n_ >> 31) & 1;
    // filter out the top bit with logical AND mask, then strip 23 unwanted bits away
    let exponent = (n_ >> 23) & 0xff;
    // retain 23 least significant bits via an AND mask
    let fraction = 0b00000000_01111111_11111111_11111111 & n_;

    (sign, exponent, fraction)
}
