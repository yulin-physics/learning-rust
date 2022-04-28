fn main() {
    println!("Hello, world!");
}

pub fn swap(numbers: &mut Vec<i32>, i: usize, j: usize) {
    let temp = numbers[i];
    numbers[i] = numbers[j];
    numbers[j] = temp;
}

// sort in place by passing mutable reference
pub fn insertion_sorter(numbers: &mut Vec<i32>) {
    for i in 1..numbers.len() {
        let mut j = i;
        while j > 0 && numbers[j - 1] > numbers[j] {
            swap(numbers, j, j - 1);
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_vector(num: i32, range: i32) -> Vec<i32> {
        (0..num)
            .map(|_| rand::thread_rng().gen_range(0..range))
            .collect()
    }

    #[test]
    fn test_insertion_sort() {
        let mut numbers: Vec<i32> = random_vector(10, 100);
        insertion_sorter(&mut numbers);
        println!("{:?}", numbers);
        for i in 0..(numbers.len() - 1) {
            assert!(numbers[i] <= numbers[i + 1])
        }
    }
}
