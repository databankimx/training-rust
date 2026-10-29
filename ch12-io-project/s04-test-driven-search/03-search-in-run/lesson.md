## Using search in run

The last step connects the tested function to the program. `run` now calls `search` and prints each line it returns, instead of printing the whole file:

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

The "Searching for" and "In file" lines are gone, so the program prints only the results. A user who searches for `frog` wants the matching line, and nothing else.

`search` collects every match before anything is printed. For a large file, that can be slower to show results than printing as each match is found. Chapter 13 returns to this
point.

---

### Try it in the runner

Choose a quick pick, and press **Go**.

* **cargo run -- frog poem.txt** prints one line: `How public, like a frog`.
* **cargo run -- body poem.txt** prints three lines, because `body` is inside `nobody` and `somebody` too.
* **cargo run -- monomorphization poem.txt** prints nothing. The word isn't in the poem, so there are no matches, and the program still exits successfully.

Note that the search is case-sensitive. A query of `Frog` wouldn't match the line that contains `frog`. The next sub-chapter adds a way to change that.

---

*For more detail, see section 12.4 "Developing the Library's Functionality with Test-Driven Development" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html).*
