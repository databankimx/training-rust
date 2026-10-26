fn first(strings: &Vec<String>) -> &String {
    &strings[0]
}

fn main() {
    let names = vec![String::from("Ferris"), String::from("Corro")];
    let f = first(&names);
    println!("First: {f}");

    // Try: move `drop(names);` to just before the println! above, so that names is dropped while f
    // still refers to it. Rust rejects the program. Why does it matter whether f is used afterwards?
    //
    // Try: change `first` so it returns a reference to a String created inside the function.
    // What does the compiler say about the returned reference?
}
