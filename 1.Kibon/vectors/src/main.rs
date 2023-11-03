fn main() {
    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    let _v2 = vec![1, 2, 3];

    let _s: &String = &v[0]; // can panic

    // let s = v.remove(0);

    let s: Option<&String> = v.get(0);
    if let Some(e) = s {
        println!("{e}");
    }

    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}");
    }

    let mut v3 = Vec::new();

    for s in v.into_iter() {
        v3.push(s);
    }

    // let i = v.get(0);
}
