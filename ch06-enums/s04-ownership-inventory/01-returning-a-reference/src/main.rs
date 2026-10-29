/// Makes a string to separate lines of text, returning a default if the provided string is blank.
/// This version returns an owned String, so the default can be returned safely.
fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        "=".repeat(10)
    } else {
        user_str.to_string()
    }
}

fn main() {
    println!("{}", make_separator(""));
    println!("{}", make_separator("Hello world!"));

    // Try: restore the original version, which returns &default where default is a local String. Which error does the compiler give?
}
