fn main() {
    // Each of these Cyrillic letters takes two bytes in UTF-8, so the first four bytes hold two letters.
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");

    // Try: change the range to &hello[0..1]. Why does the program panic, and what does the message say?
}
