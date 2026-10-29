// src/main.rs is the binary crate root. It's a separate crate, and it uses the library crate by name.
fn main() {
    println!("{}", greeter::greet("Ferris"));

    // Try: add a second binary by creating src/bin/other.rs with its own main. How many crates does the package now contain?
}
