## Solution: Several Files

This walkthrough shows one way to search several files, with errors kept on standard error. Your version may differ, as long as the behavior matches the exercise.

### The config

`Config` now holds every file path, and `build` takes them all from index 2 on:

```rust,ignore
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
        let file_paths = args[2..].to_vec();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_paths, ignore_case })
    }
}
```

`args[2..]` is a slice, and `to_vec` copies it into a vector that `Config` owns.

### The run function

`run` searches each file in turn. A file that can't be read is reported, and the loop goes on:

```rust,ignore
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
```

The `for` loop borrows `config.file_paths`, so the configuration can still be used after the loop. The file name is printed with each match, so output from several
files can be told apart.

Recording `any_failed` and returning an error at the end lets `main` set the exit code, while every file still gets searched. Returning immediately on the first
error would skip the files after it.

### The extension

The file name could be printed only when there is more than one file, by checking `config.file_paths.len() > 1` before the loop. That keeps the single-file output
unchanged.

Reading files in parallel would make the output order depend on which thread finishes first. To keep it in order, collect each file's results and print them in
the order the files were given. Chapter 16 covers threads.

---

*This walkthrough accompanies the exercise in Chapter 12 of the course. It's not from the book.*
