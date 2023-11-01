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
    let query = format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query cannot be empty"))
    } else {
        Ok(String::from("Ferris"))
    }
}
