fn invoker<T>(logic: fn(T), arg: T) {
    logic(arg);
}

fn main() {
    let greet = |name| {
        let greeting = "Nice to meet you".to_string();
        println!("{greeting} {name}")
    };

    invoker(greet, "ferris")
}
