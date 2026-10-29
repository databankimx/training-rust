fn main() {
    // push_str takes a string slice, so the caller keeps ownership of what it passes in.
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // push takes a single char.
    let mut t = String::new();
    t.push('l');
    t.push('o');
    t.push('l');
    println!("{t}");

    // to_string and String::from do the same thing for a string literal.
    let greeting = "initial contents".to_string();
    println!("{greeting}");

    // Try: use `String::from("initial contents")` instead of to_string. Does the output change?
}
