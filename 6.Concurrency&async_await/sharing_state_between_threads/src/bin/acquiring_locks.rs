use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

struct Wrapper {
    value: i32,
}

impl Wrapper {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn add(&mut self, to_add: i32) {
        self.value += to_add;
    }
}

fn main() {
    let sum = Arc::new(Mutex::new(Wrapper::new()));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..4 {
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            println!("spawn");
            let mut sum = 0;
            let start = i * 10_000 + 1;

            for num in start..start + 10_000 {
                sum += num;
            }

            let mut sum_lock = sum_clone.lock().unwrap();
            sum_lock.add(sum);
        });

        handles.push(handle);
    }

    println!("main");

    for handle in handles {
        println!("join");
        handle.join().unwrap();
    }

    let sum_lock = sum.lock().unwrap();

    println!("Sum of range 1..=40,000 : {}", sum_lock.value);
}
