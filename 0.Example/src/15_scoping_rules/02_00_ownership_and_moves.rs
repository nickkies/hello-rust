// ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is freed memory!
}

fn ownership_and_moves() {
    let x = 5u32;

    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);

    println!("a contains: {}", a);

    let b = a;

    // ownership changed
    // error
    // println!("a contains: {}", a);

    destroy_box(b);

    // error
    // println!("b contains: {}", b);
}
