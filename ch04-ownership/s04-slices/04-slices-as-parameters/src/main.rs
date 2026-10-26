fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // A slice of a String, and a reference to a String, both work with a &str parameter.
    let my_string = String::from("hello world");
    println!("{}", first_word(&my_string[..]));
    println!("{}", first_word(&my_string));

    // A string literal is already a &str, so it can be passed directly.
    let my_string_literal = "hello world";
    println!("{}", first_word(my_string_literal));

    // Try: pass first_word a String directly, without the &. Why does this fail?
}
