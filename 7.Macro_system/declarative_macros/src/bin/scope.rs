#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("My macro");
        };
    }
}

fn main() {
    my_macro!();
}

// macro_rules! my_macro {
//   () => {
//       println!("My macro");
//   };
// }
