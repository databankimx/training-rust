use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        // search returns an iterator, so each line is printed as soon as it's found, rather than after the whole file is searched.
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

// Returns an iterator, so nothing is searched until the caller asks for a line. The closure captures query, which is why both
// arguments share the lifetime 'a: the results borrow from contents, and the closure borrows from query.
pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    contents.lines().filter(move |line| line.contains(query))
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results: Vec<&str> = search(query, contents).collect();
        assert_eq!(vec!["safe, fast, productive."], results);
    }
}
