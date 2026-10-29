fn main() {
    // chars gives each Unicode scalar value, as a char.
    for c in "Зд".chars() {
        println!("{c}");
    }

    // bytes gives each raw byte, as a u8. Each Cyrillic letter here takes two bytes.
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Try: count the chars and the bytes in "नमस्ते". Which number is larger, and why?
}
