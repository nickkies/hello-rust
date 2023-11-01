fn main() {
    let username = get_username(1);

    match &username {
        Some(name) => println!("{name}"),
        None => println!("No username found"),
    }

    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(user_id: u32) -> Option<String> {
    let result = String::from("Ferris");
    let username = match user_id {
        1 => Some(result),
        _ => None,
    };

    username
}
