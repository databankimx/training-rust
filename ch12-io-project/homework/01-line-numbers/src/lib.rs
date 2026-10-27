use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Each match is printed with its line number.
    for (number, line) in search_with_line_numbers(&config.query, &contents) {
        println!("{number}: {line}");
    }

    Ok(())
}

// Returns each matching line with its number. enumerate counts from 0, so 1 is added to give the number a reader sees.
pub fn search_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((index + 1, line));
        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_numbers_are_one_based() {
        let query = "duct";
        let contents = "\
duct first
Pick three.
safe, fast, productive.
Rust:";

        assert_eq!(
            vec![(1, "duct first"), (3, "safe, fast, productive.")],
            search_with_line_numbers(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
