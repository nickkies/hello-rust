#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check it out!");
    };
    ($val:expr) => {
        println!("Check {} out!", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!("this");
}
