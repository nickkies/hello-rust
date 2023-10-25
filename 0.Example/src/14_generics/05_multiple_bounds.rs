use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn multiple_bounds() {
    let string = "word";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);

    compare_types(&array, &vec);
}
