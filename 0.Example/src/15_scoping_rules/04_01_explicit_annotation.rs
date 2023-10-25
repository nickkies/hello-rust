fn print_refs(x: &i32, y: &i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // error
    // let y: &'a i32 = &_x;
}

fn explicit_annotation() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}