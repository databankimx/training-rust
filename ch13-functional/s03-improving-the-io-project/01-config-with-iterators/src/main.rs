use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // env::args returns an iterator, and Config::build takes ownership of it, so no Vec is needed.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
