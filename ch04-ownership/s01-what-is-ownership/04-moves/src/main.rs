fn main() {
    // `first` owns the heap data for "Ferris".
    let first = String::from("Ferris");

    // Passing `first` into add_suffix moves ownership. `first` can no longer be used.
    let full = add_suffix(first);
    println!("{full}");

    // Try: uncomment the next line and run the program. Rust rejects it, because `first`
    // was moved into add_suffix. Read the error. Why is this rule there?
    // println!("{full}, originally {first}");

    // Try: call `first.clone()` and pass the copy to add_suffix instead. Then print `first` too.
    // Does the original still work?
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
