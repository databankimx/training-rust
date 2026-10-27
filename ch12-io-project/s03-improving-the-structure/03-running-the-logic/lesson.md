## Moving the Logic into run

`main` now reads the arguments, builds the configuration, and does the work. The work moves into a `run` function, so `main` is short enough to check by reading it.

```rust,ignore
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}
```

Three things changed from the earlier version:

* The return type is `Result<(), Box<dyn Error>>`. The success value is `()`, which means there's nothing to return. `Box<dyn Error>` means any error type can be
  returned, so `run` doesn't need to name the specific type. Chapter 18 covers trait objects.
* `expect` is replaced by `?`. Instead of panicking, a failed read is returned to `main`.
* The function returns `Ok(())` on success. The `()` is the idiomatic way to say the function is called for its effects only.

`main` handles the error with `if let`. There's no value to unwrap on success, so `if let` is simpler than `unwrap_or_else`:

```rust,ignore
if let Err(e) = run(config) {
    eprintln!("Application error: {e}");
    process::exit(1);
}
```

---

### Try it in the runner

Choose a quick pick, and press **Go**.

* **cargo run -- test poem.txt** prints the search terms and the poem, as before.
* **cargo run -- test missing.txt** prints `Application error:` followed by the reason the file couldn't be opened, and exits with code 1. The reason depends on the
  operating system.

---

*For more detail, see section 12.3 "Refactoring to Improve Modularity and Error Handling" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html).*
