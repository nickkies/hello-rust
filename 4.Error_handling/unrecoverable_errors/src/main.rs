fn main() {
    /// RUST_BACKTRACE=1 cargo run
    /// RUST_BACKTRACE=full cargo run
    /// panic!("Something went horribly wrong!");
    let v = vec![1, 2, 3];
    println!("{}", v[3]);
}
