use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("spawned {i}");
        }
    });

    for i in 0..10 {
        println!("main {i}");
    }

    handle.join().unwrap();
}
