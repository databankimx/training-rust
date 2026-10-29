## Writing Code to Pass the Test

The second step is the smallest change that makes the test pass. `search` goes through the text one line at a time, keeps each line that contains the query, and
returns the list:

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

Each part does one job:

* `contents.lines()` returns an iterator over the lines of the text, without their line endings.
* `line.contains(query)` checks whether the line has the query in it. Matching is case-sensitive, so `Duct` wouldn't match `duct`.
* `results.push(line)` adds a matching line to the list. Each element is a reference into `contents`, so no text is copied.

Run **Test**, and the test passes. The output should show `test tests::one_result ... ok`.

Chapter 13 covers iterators in depth, and shows a shorter way to write this function. For now, the loop shows each step clearly.

---

### Try it in the runner

Press **Test**. The test should pass.

The comments in `src/lib.rs` suggest adding a test where the query appears on two lines. The function should return both lines, in the order they appear in the text.

---

*For more detail, see section 12.4 "Developing the Library's Functionality with Test-Driven Development" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html).*
