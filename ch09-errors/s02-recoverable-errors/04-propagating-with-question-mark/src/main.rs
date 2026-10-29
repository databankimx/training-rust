use std::fs::{self, File};
use std::io::{self, Read};

// Each ? returns the error to the caller, if there is one. Otherwise it gives the value inside Ok.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // Write the file first, so the read has something to find.
    fs::write("hello.txt", "will").expect("could not write hello.txt");

    match read_username_from_file() {
        Ok(name) => println!("username: {name}"),
        Err(error) => println!("could not read username: {:?}", error.kind()),
    }

    // Try: delete the fs::write line, and delete hello.txt from this folder. Run again. Which arm runs, and why?
}
