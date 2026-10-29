## Reading an Environment Variable

The last piece is the configuration. `Config` gets an `ignore_case` field, and `Config::build` sets it from the environment variable `IGNORE_CASE`:

```rust,ignore
let ignore_case = env::var("IGNORE_CASE").is_ok();
```

`env::var` returns a `Result`. If the variable is set, to any value, it returns `Ok` with that value. If it isn't set, it returns `Err`. The program doesn't need
the value, only whether it's set, so `is_ok` is the right check. It's simpler than matching on the result, and `unwrap` or `expect` would panic on an unset variable.

`run` then uses the field to pick the function:

```rust,ignore
let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
} else {
    search(&config.query, &contents)
};
```

Both functions return the same type, so the loop that prints the lines doesn't change.

---

### Try it in the runner

Press **Go**, or choose a quick pick. The runner sets the variable for that run only. Your typed text can start with `NAME=value` settings, and they're passed to
the program as environment variables:

* **cargo run -- to poem.txt** searches case-sensitively. It prints `Are you nobody, too?` and `How dreary to be somebody!`.
* **IGNORE_CASE=1 cargo run -- to poem.txt** also prints `To tell your name the livelong day` and `To an admiring bog!`, because `To` now matches `to`.

Type `IGNORE_CASE=1 to poem.txt` in the box, and press **Go**, to try any query this way. The variable is set only for that run, so the next run without it is case-sensitive
again.

---

*For more detail, see section 12.5 "Working with Environment Variables" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html).*
