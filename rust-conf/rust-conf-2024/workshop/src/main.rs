use core::num;
use rand::prelude::SliceRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

// const NUM_THREADS:usize = 6;
const MAX_NUMBER: usize = 100_000;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2..n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

/// Send, Sync -> almost everything implements Send except Rc
/// Why are we unwrapping Mutex? Poisoning
/// Mutex penalty -> try lock free

fn main() {
    let num_cpus = num_cpus::get();
    println!("{num_cpus} cpus");
    let mut candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    let start = Instant::now();

    // let mut primes = Vec::with_capacity(10_000);
    // or bin packing
    // candidates.shuffle(&mut rand::thread_rng());

    // std::thread::scope(|scope| {
    //     let mut handles = Vec::with_capacity(num_cpus);
    //     let chunks = candidates.chunks(candidates.len() / num_cpus);

    //     for (id, chunk) in chunks.enumerate() {
    //         println!("chunck size {}", chunk.len());

    //         let handle = scope.spawn(move || {
    //             chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect()
    //         });
    //         handles.push(handle);
    //     }

    //     for handle in handles {
    //         let local_results:Vec<usize> = handle.join().unwrap();
    //         primes.extend(local_results);
    //     }
    // });

    let primes: Vec<usize> = candidates
        .par_iter()
        .filter(|n| is_prime(**n))
        .map(|n| *n)
        .collect();

    let elapsed = start.elapsed();
    println!("found {} primes", primes.len());
    println!("{:.4}", elapsed.as_secs_f32());
}
