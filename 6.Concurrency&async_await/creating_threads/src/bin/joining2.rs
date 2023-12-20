use std::{
    ops::Range,
    thread::{self, JoinHandle},
};

fn summation_thread(range: Range<u64>) -> JoinHandle<u64> {
    thread::spawn(|| {
        let mut sum = 0;
        for num in range {
            sum += num;
        }
        sum
    })
}

fn main() {
    let mut sum = 0u64;

    sum += summation_thread(1..1_000_000).join().unwrap();
    sum += summation_thread(1_000_000..2_000_000).join().unwrap();
    sum += summation_thread(2_000_000..3_000_000).join().unwrap();

    for num in 3_000_000..=4_000_000 {
        sum += num;
    }

    println!("Sum of numbers 1..=4,000,000: {sum}");
    assert_eq!(sum, 8_000_002_000_000);
}
