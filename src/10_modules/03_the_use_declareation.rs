use deeply::nested::first_function as other_function;

use deeply::nested::{
    first_function as other_first_function,
    second_function
};

fn first_function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn first_function() {
            println!("called `deeply::nested::first_function()`");
        }
        pub fn second_function() {
            println!("called `deeply::nested::second_function()`")
        }
    }
}

fn main() {
    other_function();

    println!("Entering block");
    {
        use crate::deeply::nested::first_function;

        first_function();

        println!("Leaving block");
    }

    first_function();
}
