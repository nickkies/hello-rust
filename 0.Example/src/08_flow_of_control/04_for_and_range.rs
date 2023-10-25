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
    println!();

    let names = vec!["Bob", "Frank", "Ferris"];

    // 컬렉션을 그대로 두고 루프 & 이후에 재사용 가능
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // 정확한 데이터 제공을 위해
    // 컬렉션을 사용 & 재사용 불가
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // error
    // println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];

    // 컬렉션을 제자리에서 수정 가능! (재선언 아닌듯?)
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
