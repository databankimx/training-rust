fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {word}");

    // Try: add `mut` to s, then move a call to s.clear() to just before the println! that uses word.
    // Rust rejects it, because word is still in use. Read the error. Why does the slice keep s borrowed?
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
