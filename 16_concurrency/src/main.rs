// Concurrent programming, where different parts of a program execute independently, and parallel programming, where different parts of a program execute at the same time

// Programming language-provided threads are known as green threads, green-threaded model is called the M:N model
// Rust standard library only provides an implementation of 1:1 threading

use std::thread;
use std::time::Duration;
mod channels;

fn main() {
    channels::run();
}

fn move_closures(){
    // use data from one thread in another thread
    let v = vec![1,2,3];
    let handle = thread::spawn(move||{
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

fn block_thread() {
    // the new thread will be stopped when the main thread ends, whether or not it has finished running
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn create_new_thread() {
    // the new thread will be stopped when the main thread ends, whether or not it has finished running
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
