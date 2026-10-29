## Solution: Validating the Query

This walkthrough shows one way to validate the query, and to test `Config::build`. Your version may differ.

### The check

In `Config::build`, after the length check, add:

```rust,ignore
pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    // An empty query matches every line, so it's almost certainly a mistake.
    if query.is_empty() {
        return Err("query must not be empty");
    }

    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Ok(Config { query, file_path, ignore_case })
}
```

The check comes after the length check, so `args[1]` is known to exist. Spaces-only queries are not rejected here. A query of spaces is a legitimate search for
text that contains spaces, so the choice is a design decision, and a comment should record it.

### The tests

`Config` doesn't implement `PartialEq`, so `assert_eq!` on a whole `Result<Config, _>` won't compile. Matching on the result works instead:

```rust,ignore
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
```

A helper function, `args`, turns a list of string slices into the `Vec<String>` the program uses. That keeps each test short.

Each test matches on the result. The `Ok` arm checks the fields, and the `Err` arm checks the message. A failing `panic!` in the wrong arm says what was expected.

### The extension

A file path that is empty is also a mistake, and checking it in `Config::build` keeps all argument checks in one place. Checking whether the file exists is a
different kind of check, because it touches the file system. That belongs in `run`, where the file is read, so the error can name the file.

---

*This walkthrough accompanies the exercise in Chapter 12 of the course. It's not from the book.*
