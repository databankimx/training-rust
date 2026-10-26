fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // The index in word is no longer tied to the string, so clearing s leaves word pointing at nothing.
    s.clear();
    println!("The index is {word}, but the string is now '{s}'");

    // Try: print the first word using the index before calling s.clear(). Does the output change?
    // Try: change the text in s to "hi there" without changing the index. What happens to the first word?
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
