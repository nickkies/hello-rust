use std::{thread, time::Duration};

fn main() {
    let s = "Hi Ferris!".to_string();

    let handle = thread::spawn(move || {
        println!("{s}");

        for i in 0..20 {
            println!("Spawned Thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("Main Thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
