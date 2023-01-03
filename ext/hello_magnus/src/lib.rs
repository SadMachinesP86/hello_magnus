extern crate magnus;

use magnus::{define_global_function, function};

fn hello_magnus() {
    println!("Hello, Magnus!");
}

#[magnus::init]
fn init() {
    define_global_function("hello_magnus", function!(hello_magnus, 0));
}
