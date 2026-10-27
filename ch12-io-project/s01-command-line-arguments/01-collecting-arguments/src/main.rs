use std::env;

fn main() {
    // args holds every argument. The first item is the path of the program itself.
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");

    // Try: run with no arguments, then with two. How many items does the vector have each time?
}
