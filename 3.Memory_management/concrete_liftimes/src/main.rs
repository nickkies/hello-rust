fn main() {
    let s1 = "Ferris".to_string();
    println!("s1: {s1}");

    let s2 = s1;
    // println!("s1: {s1}");

    let r1: String = Default::default();
    {
        let s1 = "Ferris".to_string();
        // r1 = &s1;
    }

    println!("r1: {r1}");

    let mut s1 = "Ferris".to_string();
    let r1 = &s1;
    let r2 = &mut s1;
    println!("r1: {r1}");
    r2.push_str("!");
}
