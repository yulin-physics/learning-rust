use std::collections::HashMap;

pub fn find_mode(numbers: &Vec<u32>) -> u32 {
    let mut count_map = HashMap::new();
    for number in numbers {
        let count = count_map.entry(number).or_insert(0);
        *count += 1;
    }
    let mut mode = (0, 0);
    for (number, count) in count_map {
        if count > mode.1 {
            mode = (*number, count)
        }
    }
    return mode.0;
}

pub fn find_median(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort();
    let med = numbers.get(numbers.len() / 2);
    match med {
        Some(number) => *number,
        None => 0,
    }
}
