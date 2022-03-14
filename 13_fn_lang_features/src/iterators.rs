// In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up

pub fn run() {
    let v1 = vec![1, 2, 3];
    // immutable refs to v1 values
    let mut v1_iter = v1.iter();
    // takes ownership of v1
    // let v1_iter = v1.into_iter();
    // mutable refs of v1 values
    // let v1_iter = v1.iter_mut();

    //  calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    for val in v1_iter {
        println!("Got {}", val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        // Methods that call next are called consuming adaptors, because calling them uses up the iterator.
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_chain() {
        // Iterator adaptors, allow you to change iterators into different kinds of iterators.
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}
