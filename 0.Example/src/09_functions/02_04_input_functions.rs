fn call_me<F: Fn()>(f: F) {
    f();
}

fn my_function() {
    println!("I'm a function!");
}

fn input_functions() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(my_function);
}
