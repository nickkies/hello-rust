use std::{
    sync::mpsc::{self, Sender},
    thread,
};

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let nums: [i32; 8] = [12, 43, 54, 43, 53, 52, 98, 89];
    sum(&nums, 3, tx);
    let mut res = 0;
    for sum in rx {
        res += sum;
    }
    println!("Sum of numbers: {res}");
}

fn sum(nums: &[i32], thread_count: usize, tx: Sender<i32>) {
    let elements_per_thread = (nums.len() as f32 / thread_count as f32).ceil() as usize;
    let mut start_pos: usize;
    let mut partition: Vec<i32>;

    for i in 0..thread_count - 1 {
        start_pos = i * elements_per_thread;
        partition = Vec::from(&nums[start_pos..start_pos + elements_per_thread]);

        let tx_clone = tx.clone();
        thread::spawn(move || {
            let mut sum = 0;
            for num in partition {
                println!("{num}, tx_clone, {start_pos}");
                sum += num;
            }
            tx_clone.send(sum).unwrap();
        });
    }

    partition = Vec::from(&nums[(thread_count - 1) * elements_per_thread..]);
    thread::spawn(move || {
        let mut sum = 0;
        for num in partition {
            println!("{num}, tx");
            sum += num;
        }
        tx.send(sum).unwrap();
    });
}
