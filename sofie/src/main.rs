use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Sofie;

fn main() {
    Sofie::hello_macro();
}
