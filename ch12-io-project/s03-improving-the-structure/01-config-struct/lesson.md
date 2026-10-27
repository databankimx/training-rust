## Grouping the Configuration

The program now has two jobs in `main`: it reads the arguments, and it does the work. The first refactoring separates them. The argument handling moves into a
function, `parse_config`, and the values it returns are grouped into a `Config` struct.

```rust,ignore
struct Config {
    query: String,
    file_path: String,
}
```

`parse_config` makes a `Config` from the arguments, and `main` uses the struct's fields by name:

```rust,ignore
let config = parse_config(&args);
println!("Searching for {}", config.query);
```

The fields have names that say what they're for, so a reader of `main` knows that `query` and `file_path` belong together. The values are copied with `clone`,
because `Config` owns its strings, and `args` still owns the originals. That's a small cost, and the book accepts it at this stage, to keep the code simple.

The output is the same as before:

```
Searching for test
In file poem.txt
With text:
...
```

---

### Try it in the runner

Choose the quick pick, or type `test poem.txt` in the box, and press **Go**. The output matches the last section.

The comments in `src/main.rs` suggest making `parse_config` fail when there are too few arguments. The current code still panics with the index error, which the next
step fixes.

---

*For more detail, see section 12.3 "Refactoring to Improve Modularity and Error Handling" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html).*
