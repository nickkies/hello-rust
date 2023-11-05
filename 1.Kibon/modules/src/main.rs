use modules::Credentials;

fn main() {
    let creds = Credentials {
        username: "Ferris".to_owned(),
        password: "pwd".to_owned(),
    };

    modules::authenticate(creds);
}
