use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn is_prime(num: i32) -> bool {
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }

    if num <= 1 {
        false
    } else {
        true
    }
}

fn main() {
    let primes: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    let thread_count = 5;
    let elements_per_thread = 100_000 / thread_count;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..thread_count {
        let start = i * elements_per_thread;
        let list_clone = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            for num in start..start + elements_per_thread {
                if is_prime(num) {
                    let mut lock = list_clone.lock().unwrap();
                    lock.push(num);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let lock = primes.lock().unwrap();

    println!("Prime numbers: {lock:?}");
    println!("Prime numbers count: {}", lock.len());
}
