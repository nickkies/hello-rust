fn main() {
    let mut s1 = String::from("Rust");
    let r1 = &s1;
    println!(r1);
    let r2 = &mut s1;
    add_to_string(r2);
    println!("s1 is: {s1}");
    let _s2 = generate_string();
}

fn generate_string() -> String {
    String::from("ferris");
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome!");
}

fn prtin_string(p1: &String) {
    println!("{p1}");
}
