use std::fmt::Result;
use std::io::Result as IoResult;

// fmt::Result is Result<(), fmt::Error>. io::Result is renamed IoResult, so the two don't clash.
fn fmt_ok() -> Result {
    Ok(())
}

fn io_ok() -> IoResult<()> {
    Ok(())
}

fn main() {
    println!("{:?} {:?}", fmt_ok(), io_ok());

    // Try: change `use std::io::Result as IoResult;` to `use std::io::Result;`. What does the compiler say about the name Result?
}
