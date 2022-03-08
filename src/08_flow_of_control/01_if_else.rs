fn fi_else() {
    let i = 51;
    let f = 51f64;

    // 25
    println!("Case i32: {} -> {} ", i, fn_i32(i));
    // 25.5
    println!("Case f64: {}, -> {}", f, fn_f64(f));
}

fn fn_i32(n:i32) -> i32 {
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    }
}

fn  fn_f64(n:f64) -> f64 {
    if n < 0.0 {
        print!("{} is negative", n);
    } else if n > 0.0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    if n < 10.0 && n > -10.0 {
        10.0 * n
    } else {
        n / 2.0
    }
}
