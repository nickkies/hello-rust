fn guards() {
    let pair = (3, 0);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _y) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"),
    }
}
