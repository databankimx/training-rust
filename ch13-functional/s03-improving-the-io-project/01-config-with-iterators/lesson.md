## Config with Iterators

The first version of `Config::build` took a slice of strings, and cloned the query and file path so that `Config` could own them. Cloning allocates new memory, and it's
wasted work when the caller could give up its values. This step changes `build` to take ownership of an iterator instead.

```rust,ignore
pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
    };
    // ...
}
```

Three changes make this work:

* The parameter is `impl Iterator<Item = String>`, so any iterator of strings can be passed. `env::args` returns one, so `main` passes it directly. Chapter 10 covered
  `impl Trait` in parameters.
* `mut` is needed, because `next` changes the iterator's position.
* `args.next()` gives the strings one at a time, so each one can be moved into `Config`, with no `clone`. The first call skips the program name.

Each missing argument returns an `Err`, with a message that says which argument is missing, and the `match` on `next` handles the `None` case.

---

### Try it in the runner

Choose a quick pick, and press **Go**, or press **Run**.

* **cargo run -- to poem.txt** prints the two lines that contain `to`, as before.
* **cargo run** with no arguments prints `Problem parsing arguments: Didn't get a query string`, and exits with code 1.

The program behaves as it did in Chapter 12, but `Config::build` no longer copies any strings.

---

*For more detail, see section 13.3 "Improving Our I/O Project" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html).*
