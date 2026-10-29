## Clarifying Search with Iterator Adapters

The `search` function in the earlier version used a loop, a mutable `results` vector, and `push`. Iterators can say the same thing more directly. The purpose of `search`
is to keep the lines that contain the query, so the code can say exactly that:

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}
```

Read left to right, this is the whole function:

* `lines` gives each line of `contents`.
* `filter` keeps the lines for which the closure returns `true`. The closure checks each line for the query.
* `collect` gathers the kept lines into a `Vec`.

The result is the same as the loop, and there's no mutable state to get wrong. Keeping the state out of the function also makes it easier to run it in parallel later,
because nothing is shared between the steps.

The book suggests the same change for `search_case_insensitive`, and this sample leaves it as a loop, so you can compare the two styles.

---

### Try it in the runner

Choose a quick pick, and press **Go**, or press **Run**.

* **cargo run -- to poem.txt** prints the same two lines as before.
* **cargo run -- body poem.txt** prints the three lines that contain `body`.

Then press **Test**. The test passes, and it's the same test as before, because the function returns the same result.

---

*For more detail, see section 13.3 "Improving Our I/O Project" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html).*
