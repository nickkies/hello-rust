fn main() {
    // definition
    let a = 5;

    println!("a is: {a}");

    // mutability
    let mut b = 5;
    b = 10;

    println!("b is now: {b}");

    // shadowing
    let c = 10;
    let c = 20;

    println!("c is now: {c}");

    // scope
    let d = 30;

    {
        let d = 40;
        println!("d is now: {d}");
    }

    println!("d is still: {d}");
}
