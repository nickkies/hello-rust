fn main() {
    // if else
    let a = 5;

    if a > 5 {
        println!("bigger then 5");
    } else if a > 3 {
        println!("bigger then 3");
    } else {
        println!("smaller then 3");
    }

    let _b = if a > 5 { 1 } else { -1 };

    // loop
    let x = loop {
        break 5;
    };

    println!("{x}");

    let mut a = 0;

    while a < 5 {
        println!("a is {a}");
        a += 1;
    }

    // for loop
    let a = [1, 2, 3, 4, 5];

    for el in a {
        println!("el is {el}");
    }
}
