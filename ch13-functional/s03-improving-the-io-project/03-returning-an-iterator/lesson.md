## Returning an Iterator from search

The last improvement is to have `search` return an iterator, rather than a `Vec`. Nothing is searched until the caller asks for a line:

```rust,ignore
pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    contents.lines().filter(move |line| line.contains(query))
}
```

The return type `impl Iterator<Item = &'a str>` says the function returns some iterator of string slices. The caller doesn't need to know the exact type, which is
a long chain of `Lines` and `Filter`.

The closure captures `query`, so the returned iterator uses `query` as well as `contents`. Both arguments therefore share the lifetime `'a`, which tells the compiler the
results can't outlive either of them. The `move` keyword gives the closure its own copy of the reference to `query`, so it can run later.

`run` now prints the lines as the iterator produces them:

```rust,ignore
for line in search(&config.query, &contents) {
    println!("{line}");
}
```

With a large file, the first matches appear on screen while the rest of the file is still being searched. The collected version waited until every match was found.
The `search_case_insensitive` function still returns a `Vec`, so `run` keeps a separate loop for it.

The test now collects the iterator into a `Vec` to compare it with the expected result.

---

### Try it in the runner

Choose a quick pick, and press **Go**, or press **Run**.

* **cargo run -- body poem.txt** prints the three lines that contain `body`.
* **IGNORE_CASE=1 cargo run -- to poem.txt** prints four lines, because the case-insensitive search also matches `To`.

Press **Test** to check that the test still passes. The poem is too short to show the lazy printing, so try the same search on a much longer file in a terminal, if you
have one. Lines appear as they're found, rather than all at once at the end.

---

*For more detail, see section 13.3 "Improving Our I/O Project" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html).*
