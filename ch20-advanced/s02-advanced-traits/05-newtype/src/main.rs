use std::fmt;

// A newtype: a tuple struct with one field. Wrapper is defined in this crate, so Display can be implemented for it.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 is the Vec inside the wrapper.
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    // Try: call w.len(). Why does it fail, and what would you add to make it work?
}
