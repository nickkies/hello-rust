unsafe trait Length {
    fn length(&self) -> usize;
}

unsafe impl Length for String {
    fn length(&self) -> usize {
        self.len()
    }
}

unsafe impl Length for i32 {
    fn length(&self) -> usize {
        match self {
            -9..=9 => 1,
            _ => 1 + (self / 10).length(),
        }
    }
}

fn main() {
    let str = "Unsafe Traits".to_string();
    let num = 12345;

    println!("\"{}\" takes {} bytes", str, str.length());
    println!("\"{}\" has {} digits", num, num.length());
}
