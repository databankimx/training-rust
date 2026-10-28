// The trait comes from hello_macro, and the derive comes from hello_macro_derive. Both names are HelloMacro, and they live in different namespaces.
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// The derive generates the impl of HelloMacro for Pancakes, so no impl block is written here.
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();

    // Try: delete the line that imports the trait, use hello_macro::HelloMacro;. What does the compiler say about the generated code?
}
