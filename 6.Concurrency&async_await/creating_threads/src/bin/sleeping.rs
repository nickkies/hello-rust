use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        println!("Count down from 10 to 1 🚀");
        for num in (1..=10).rev() {
            println!("Count down: {num}");
            thread::sleep(Duration::from_millis(500));
        }
    });
    println!("Count up from 1 to 10...");
    for num in 1..=10 {
        println!("Count up: {num}");
    }
    thread::sleep(Duration::from_millis(5_000));
}
