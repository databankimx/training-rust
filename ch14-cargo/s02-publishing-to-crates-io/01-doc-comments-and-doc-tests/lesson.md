## Documentation Comments and Doc Tests

Rust has a kind of comment for documentation. A documentation comment starts with three slashes, `///`, and it's placed just before the item it describes. It supports
Markdown, so headings and code blocks work. The comments describe how to *use* a public item, not how it's built.

```rust,ignore
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

The `# Examples` heading starts a section in the generated documentation. Running `cargo doc --open` builds the HTML, and shows the example as part of the function's page.
Other common headings are `# Panics`, for when the function can panic, `# Errors`, for the kinds of error a `Result` can hold, and `# Safety`, for `unsafe` functions.

---

### Doc tests

A code block inside a documentation comment is also a test. `cargo test` compiles and runs it, so an example can't quietly go out of date. The example above is run as a
doc test, and it passes when the `assert_eq!` is true.

The **Test** button shows that section, along with the unit test in the same file. Its output looks like this, with the line number of the example, which depends on the file:

```
   Doc-tests my_crate

running 1 test
test src\lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

The example's code block starts on line 5 of `src/lib.rs`, so the test is named after that line. If you change the function, or the example, so that the `assert_eq!`
fails, the doc test fails too. That's the point: the documentation is checked every time the tests run.

---

### Try it in the runner

Press **Run** for the program's output, which is `add_one(2) = 3`. Then press **Test** for the doc test section.

The comments in `src/main.rs` suggest changing the example in `src/lib.rs` to expect 7. Then **Test** reports the doc test as FAILED, with the values that were compared.

---

*For more detail, see section 14.2 "Publishing a Crate to Crates.io" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html).*
