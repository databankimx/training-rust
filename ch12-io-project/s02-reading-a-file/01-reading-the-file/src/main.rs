use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // read_to_string reads the whole file into a String. expect stops the program with this message if the read fails.
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    // Try: give a file name that doesn't exist. What does the panic message say?
}
