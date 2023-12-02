struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let _validator = |username: &str, password: &str| !username.is_empty() && !password.is_empty();
    let weak_password = "password123!".to_string();

    let validator2 = |username: &str, password: &str| {
        !username.is_empty()
            && !password.is_empty()
            && password.len() > 8
            && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
            && password != weak_password
    };

    println!("{weak_password}");

    let creds = Credentials {
        username: "admin".to_string(),
        password: "password123!".to_string(),
        validator: validator2,
    };
    println!("{}", creds.is_valid());
}

fn _validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
