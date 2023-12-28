use std::time::Duration;

use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];

    handles.push(tokio::spawn(async {
      let _ = tokio::task::spawn_blocking(|| {
          expensive_computation();
      }).await;
    }));

    for i in 0..=1 {
        let handle = tokio::spawn(async move {
            async_fn(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn async_fn(i: i32) {
    println!("[{i}] I'm async function!");
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}] First result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_string()
}

fn expensive_computation() {
  let mut sum = 0;
  for _ in 0..100_000_000 {
    sum += 1;
  }
  println!("Done with expensive computation! sum = {sum}");
}
