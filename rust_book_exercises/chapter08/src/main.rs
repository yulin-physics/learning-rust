mod ints;
mod pig_latin;

fn main() {
    println!("=== Find Median of Integers ===");
    let mut numbers = vec![2, 3, 4, 7, 5, 10, 14, 14];
    let median = ints::find_median(&mut numbers);
    println!("median of {:?} is {}", numbers, median);

    println!("=== Find Mode of Integers ===");
    let mode = ints::find_mode(&numbers);
    println!("mode of {:?} is {}", numbers, mode);

    println!("=== Pig Latin ===");
    let word = "Friday";
    println!("Pig latin of {} is {}", word, pig_latin::convert(word));
}
