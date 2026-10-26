fn main() {
    let mut s = String::from("Hello");

    // Borrowing s with s_ref removes write permission from s, but only while s_ref is in use.
    let s_ref = &s;
    println!("{s_ref}");
    println!("{s}");

    // s_ref is not used after this point, so s gets its write permission back.
    s.push_str(" world");
    println!("{s}");

    // Try: add `println!("{s_ref}");` after the push_str. Which permission does Rust say is missing?
}
