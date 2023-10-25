fn primitives() {
    // type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0;     // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // default
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    let mut inferred_type = 12; // `i32`
    inferred_type = 4294967296i64;   // `i64`

    let mut mutable = 12;
    mutable = 21;

    // Error
    // mutable = true;

    // overwritten
    let mutable = true;

    println!("{}", mutable);
}
