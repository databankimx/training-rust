## Returning a Result Instead of Panicking

The last step still panicked with `index out of bounds` when the arguments were missing. That message is written for programmers, so it doesn't help a user. This
step checks the number of arguments, and returns the problem as a `Result`.

`Config::build` replaces `parse_config`. It's an associated function, so it's called as `Config::build`, and it returns `Result<Config, &'static str>`:

```rust,ignore
fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    // ...
    Ok(Config { query, file_path })
}
```

The name is `build` rather than `new`, because many programmers expect `new` to never fail.

`main` handles the result with `unwrap_or_else`. If the result is `Ok`, the `Config` is returned. If it's `Err`, the closure prints the message and exits with code 1,
which signals to the program that started this one that something went wrong:

```rust,ignore
let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
});
```

`eprintln!` writes to standard error, which the runner shows in its Output pane.

---

### Try it in the runner

Choose a quick pick, and press **Go**.

* **cargo run** prints `Problem parsing arguments: not enough arguments`, and the run finishes with exit code 1. There's no index error, and no backtrace.
* **cargo run -- test poem.txt** prints the same output as before.

---

*For more detail, see section 12.3 "Refactoring to Improve Modularity and Error Handling" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html).*
