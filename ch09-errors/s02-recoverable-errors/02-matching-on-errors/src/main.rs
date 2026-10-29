use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("missing.txt");

    match file_result {
        Ok(_file) => println!("missing.txt opened"),
        Err(error) => match error.kind() {
            // A missing file is an expected case, so the program carries on.
            ErrorKind::NotFound => println!("missing.txt does not exist yet"),
            // Any other reason is not something this program can handle, so it panics.
            other => panic!("could not open missing.txt: {other:?}"),
        },
    }

    // Try: change "missing.txt" to a file name that exists in this folder. Which arm runs now?
}
