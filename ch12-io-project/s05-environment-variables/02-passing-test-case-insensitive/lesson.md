## Implementing search_case_insensitive

The new function is almost the same as `search`. The only difference is that both the query and each line are converted to lowercase before the comparison:

```rust,ignore
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
```

Lowercasing the query means `rust`, `RUST`, `Rust`, and `rUsT` all search for the same text. Lowercasing each line means the case of the text doesn't matter either.

Two details are worth noticing:

* `query` is shadowed by a new `String`. `to_lowercase` creates new text, so there is no existing string slice to use, and the new `String` must be borrowed with `&` when it's passed to `contains`.
* The returned lines are still the original lines, not the lowercased copies. `results.push(line)` pushes the line from `contents`, so the output keeps its capitals.

`to_lowercase` handles basic Unicode, but not every case mapping. A full solution for all languages is a larger job, and the book leaves it aside.

Run **Test**, and both tests pass.

---

### Try it in the runner

Press **Test**. Both `case_sensitive` and `case_insensitive` should pass.

The comments in `src/lib.rs` suggest searching for `DUCT` with the new function. Think about which lines would match, and why. The line `Duct tape.` is one that the case-sensitive search skips.

---

*For more detail, see section 12.5 "Working with Environment Variables" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html).*
