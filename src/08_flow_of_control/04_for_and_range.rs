fn for_and_range() {
    //for n in 1..101 {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz ");
        } else if n % 3 == 0 {
            print!("fizz ");
        } else if n % 5 == 0 {
            print!("buzz ");
        } else {
            print!("{} ", n);
        }
    }
}
