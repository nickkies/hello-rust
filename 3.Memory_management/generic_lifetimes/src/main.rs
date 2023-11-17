fn main() {
    let player1 = "player 1".to_string();
    let player2 = "player 2".to_string();

    let result = first_turn(&player1, &player2);
    println!("Player going first is: {result}");

    let player1 = "player 1".to_string();
    {
        let player2 = "player 2".to_string();
        let result = first_turn(&player1, &player2);
        println!("Player going first is: {result}");
    }

    let player1 = String::from("player 1");
    let result;
    {
        let player2 = String::from("player 2");
        result = p1_first(&player1, &player2);
    }
    println!("Player going first is: {result}");

    let result = static_lifetime(&player1, &player2);
    println!("{result}");
}

fn first_turn<'a>(p1: &'a String, p2: &'a String) -> &'a String {
    if rand::random() {
        p1
    } else {
        p2
    }
}

fn p1_first<'a>(p1: &'a String, _: &String) -> &'a String {
    p1
}

fn static_lifetime(_: &String, _: &String) -> &'static str {
    let s: &'static str = "static life time";
    s
}
