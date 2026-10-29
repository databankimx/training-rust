use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Index 0 is the program path, so the arguments start at index 1.
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // Try: run with no arguments. Which index is out of bounds, and what does the message say?
}
