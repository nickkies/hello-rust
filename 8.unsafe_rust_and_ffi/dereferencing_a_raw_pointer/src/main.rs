fn main() {
    let mut s = "Ferris".to_string();

    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;

    let address = 0x012345usize;
    let _raw3 = address as *const String;

    unsafe {
        (*raw2).push_str("!!!");
        println!("raw1 is: {}", *raw1);
    }
}
