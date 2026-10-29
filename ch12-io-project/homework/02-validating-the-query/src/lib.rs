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

        // An empty query matches every line, so it's almost certainly a mistake. Spaces-only queries are allowed,
        // because they are a legitimate search for text that contains spaces.
        if query.is_empty() {
            return Err("query must not be empty");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
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

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    #[test]
    fn valid_query_and_file() {
        match Config::build(&args(&["minigrep", "body", "poem.txt"])) {
            Ok(config) => {
                assert_eq!(config.query, "body");
                assert_eq!(config.file_path, "poem.txt");
            }
            Err(e) => panic!("expected Ok, got {e}"),
        }
    }

    #[test]
    fn empty_query_is_an_error() {
        match Config::build(&args(&["minigrep", "", "poem.txt"])) {
            Ok(_) => panic!("an empty query should be rejected"),
            Err(message) => assert_eq!(message, "query must not be empty"),
        }
    }

    #[test]
    fn too_few_arguments_is_an_error() {
        match Config::build(&args(&["minigrep", "body"])) {
            Ok(_) => panic!("a missing file path should be rejected"),
            Err(message) => assert_eq!(message, "not enough arguments"),
        }
    }
}
