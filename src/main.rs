use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes {
    marmot: String,
    foo: i32,
}

fn main() {
    Pancakes::hello_macro();
}