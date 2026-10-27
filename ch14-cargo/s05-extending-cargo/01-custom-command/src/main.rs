use std::env;

fn main() {
    // args holds the program path, then "hello" (passed by Cargo for the subcommand name), then anything typed after it.
    // skip(2) leaves out both of the first two, so only the extra words remain.
    let args: Vec<String> = env::args().skip(2).collect();
    println!("Hello from a custom Cargo command!");
    println!("Arguments: {args:?}");

    // Try: in a terminal, install this with cargo install --path . and run cargo hello extra words.
}
