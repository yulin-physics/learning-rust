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
