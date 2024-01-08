unsafe trait MyTrait {
    fn some_funtion(&self);
}

unsafe impl MyTrait for String {
    fn some_funtion(&self) {
        // ...
    }
}

fn main() {
    let s = "some string".to_string();
    s.some_funtion();
}
