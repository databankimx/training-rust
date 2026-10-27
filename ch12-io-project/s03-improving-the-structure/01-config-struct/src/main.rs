use std::env;
use std::fs;

// The two values the program is configured with, grouped so their purpose is clear.
struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    // Try: change the body of parse_config so it fails when args has fewer than three items. What message do you see?
}

// Turns the arguments into a Config. The arguments are copied, because Config owns its strings.
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}
