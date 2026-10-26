fn main() {
    let s = String::from("hello world");

    // Each slice is a reference to part of s, written with a range in square brackets.
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{hello} {world}");

    // A &String is a plain pointer. A &str is a pointer plus a length, so it is twice the size.
    println!("{} {}", std::mem::size_of::<&String>(), std::mem::size_of::<&str>());

    // Try: change 0..5 to 1..4. What does hello contain now?
    // Try: write `let word = &s[..];` and print it. What does the empty start index mean?
}
