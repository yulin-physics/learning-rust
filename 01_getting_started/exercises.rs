fn main() {
    let s = "hi";
    let reverse = reverse_string(s);
    println!("string: {}, {}", s, reverse);
}

fn reverse_string(s: &str) -> String {
    warn("hi");
    s.chars().rev().collect()
}
