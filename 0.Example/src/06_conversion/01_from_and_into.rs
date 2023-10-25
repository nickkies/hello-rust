use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn form_and_into() {
    let from_num = Number::from(30);
    println!("My from number is {:?}", from_num);

    let int = 5;
    let into_num: Number = int.into();
    println!("My into number is {:?}", into_num);
}
