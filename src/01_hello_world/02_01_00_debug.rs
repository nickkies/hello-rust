#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn debug() {
    // {:?} 랑 {} 비슷
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Shristian",
             actor = "actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Nick";
    let age = 30;
    let nick = Person { name, age };

    // Pretty print
    println!("{:#?}", nick);
}
