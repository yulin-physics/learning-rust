pub fn run() {
    let s = String::from("good morning");

    let word = first_word(&s[..]);

    //string slices - of type "&str"
    // The value is made up of a reference to the starting point of the slice and the number of elements in the slice.
    let good = &s[..4];
    let morning = &s[5..s.len()];

    println!("{}, {}, {}", word, good, morning);

    let a = [1, 2, 3];
    let slice = &a[1..];
    assert_eq!(slice, &[2, 3])
}

// string literals &str *are* string slices already
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // go through the String element by element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
