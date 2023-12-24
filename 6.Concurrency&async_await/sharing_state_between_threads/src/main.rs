use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use rand::Rng;

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Self {
        Self {
            connections: vec![],
        }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 0..10 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let secs = rng.gen_range(0.0..1.0);
            thread::sleep(Duration::from_secs_f64(secs));

            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let db_lock = db.lock().unwrap();

    println!("{db_lock:?}");
}
