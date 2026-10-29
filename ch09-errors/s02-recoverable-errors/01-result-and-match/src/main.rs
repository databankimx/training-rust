use std::fs::File;

fn main() {
    // File::open returns a Result. Ok holds the file, and Err holds the reason it failed.
    let greeting_file_result = File::open("hello.txt");

    match greeting_file_result {
        Ok(_file) => println!("hello.txt opened"),
        Err(error) => println!("could not open hello.txt: {:?}", error.kind()),
    }

    // Try: create a file called hello.txt in this project folder, and run again. What changes?
}
