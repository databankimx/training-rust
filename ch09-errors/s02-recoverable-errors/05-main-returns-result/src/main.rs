use std::error::Error;
use std::fs::{self, File};
use std::io::Read;

// Box<dyn Error> means "any kind of error", so ? works for both io::Error results here.
fn main() -> Result<(), Box<dyn Error>> {
    fs::write("hello.txt", "will")?;

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    println!("username: {username}");

    // Returning Ok(()) makes the program exit with code 0.
    Ok(())
}
