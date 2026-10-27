use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Returns an error message, rather than panicking, when there are too few arguments.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else runs the closure only when build returns an error. The closure prints the message, and exits with code 1.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    // Try: run with no arguments. What is printed, and what exit code does the program return?
}
