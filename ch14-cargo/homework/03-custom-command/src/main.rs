use std::env;
use std::fs;
use std::process;

fn count_lines(text: &str) -> usize {
    text.lines().count()
}

fn main() {
    // Cargo passes the program path, then the subcommand name "lines", then the file names.
    let files: Vec<String> = env::args().skip(2).collect();

    if files.is_empty() {
        eprintln!("usage: cargo lines <file>...");
        process::exit(1);
    }

    let mut total = 0;
    let mut failed = false;

    for file in &files {
        match fs::read_to_string(file) {
            Ok(text) => {
                let count = count_lines(&text);
                total += count;
                println!("{file}: {count}");
            }
            Err(error) => {
                // A file that can't be read is reported on standard error, and the others are still counted.
                eprintln!("could not read {file}: {error}");
                failed = true;
            }
        }
    }

    if files.len() > 1 {
        println!("total: {total}");
    }

    if failed {
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_three_lines() {
        assert_eq!(count_lines("a\nb\nc"), 3);
    }

    #[test]
    fn empty_text_has_no_lines() {
        assert_eq!(count_lines(""), 0);
    }
}
