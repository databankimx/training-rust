use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
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

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // run returns Ok(()) on success. An error is printed, and the program exits with code 1.
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// All of the program's work happens here. Errors are returned to main, not handled with expect.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // The ? operator returns any error from read_to_string to main.
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}

    // Try: give a file that doesn't exist. What does the Application error line say?
