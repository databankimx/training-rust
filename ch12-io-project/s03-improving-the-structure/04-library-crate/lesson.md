## Splitting Code into a Library Crate

The program's logic now lives in `run`, and the configuration in `Config`. Both are moved into `src/lib.rs`, which makes them part of a library crate:

```rust,ignore
// src/lib.rs
pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ...
}
```

The `pub` keyword makes the items visible outside the library. `main.rs` is a separate crate, and it can only use what the library makes public. The fields of
`Config` are `pub` as well, so `main` and other code can read them.

`main.rs` now calls the library:

```rust,ignore
use minigrep::Config;

let config = Config::build(&args).unwrap_or_else(|err| { /* ... */ });
if let Err(e) = minigrep::run(config) { /* ... */ }
```

The binary and the library have the same name, `minigrep`. Cargo supports that, and `main` refers to the library with `minigrep::`.

Nothing about the output changes. The point of this step is the structure: the logic is now in a library, and the next sub-chapter writes tests against it. A
library can be tested in ways that `main` can't, because `main` can't be called from a test.

---

### Try it in the runner

Choose a quick pick, and press **Go**.

* **cargo run -- test poem.txt** prints the same output as the last step.
* **cargo run** prints `Problem parsing arguments: not enough arguments`, and exits with code 1.

---

*For more detail, see section 12.3 "Refactoring to Improve Modularity and Error Handling" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html).*
