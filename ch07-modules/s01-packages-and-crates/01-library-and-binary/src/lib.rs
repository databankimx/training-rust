// src/lib.rs is the library crate root. Its name is the package name, greeter.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}, from the library crate!")
}
