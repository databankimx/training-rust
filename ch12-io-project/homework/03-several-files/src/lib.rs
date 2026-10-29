use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        // Every argument after the query is a file path.
        let file_paths = args[2..].to_vec();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_paths, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut any_failed = false;

    for path in &config.file_paths {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let results = if config.ignore_case {
                    search_case_insensitive(&config.query, &contents)
                } else {
                    search(&config.query, &contents)
                };
                for line in results {
                    println!("{path}: {line}");
                }
            }
            Err(error) => {
                // A missing file is reported on standard error, and the other files are still searched.
                eprintln!("Could not read {path}: {error}");
                any_failed = true;
            }
        }
    }

    if any_failed {
        return Err("some files could not be read".into());
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

    #[test]
    fn builds_config_with_several_files() {
        let args: Vec<String> = ["minigrep", "body", "poem.txt", "other.txt"].iter().map(|s| s.to_string()).collect();
        let config = Config::build(&args).expect("two file paths should build");
        assert_eq!(config.file_paths, vec!["poem.txt", "other.txt"]);
    }
}
