#[tokio::main]
async fn main() {
    let f = async_fn();
    println!("Let's go!");
    f.await;
}

async fn async_fn() {
    println!("I'm an async function!");
    let s1 = read_from_database().await;
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}

async fn read_from_database() -> String {
    "DB Result".to_string()
}
