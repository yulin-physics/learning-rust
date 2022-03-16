// Shared memory concurrency similar to multiple ownership
// Multiple threads can access the same memory location at the same time

//  Rc<T> is not safe to share across threads. When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped. But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread. 

// Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. 

// atomics is a concurrency primitive, work like primitive types but are safe to share across threads.

use std::sync::{ Mutex, Arc};
use std::thread;

pub fn run() {
    multi_threads()
}

fn multi_threads() {
    // Mutex<T> provides interior mutability, as the Cell family does
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn mutex_api() {
    let m = Mutex::new(5);
    {
        // acquire the local, returns MutexGuard smart pointer
        let mut num = m.lock().unwrap();
        *num = 6;

        //releases lock automatically when MutexGuard goes out of scope
    }

    println!("m = {:?}", m);
}
