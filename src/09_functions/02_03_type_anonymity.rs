fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn type_anonymity() {
    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}
