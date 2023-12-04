fn main() {
    let my_str = "Hi there!".to_string();
    let substr = "here";

    println!("{my_str}");

    let check_substr = move |sub: &str| my_str.contains(sub);
    let res = check_substr(substr);

    println!("{substr} : {res}");
}
