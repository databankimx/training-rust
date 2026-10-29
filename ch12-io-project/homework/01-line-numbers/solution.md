## Solution: Line Numbers

This walkthrough shows one way to write the line-number search. Your version may differ, as long as the output is the same.

### The new function

Add this to `src/lib.rs`, next to `search`:

```rust,ignore
pub fn search_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    // enumerate counts from 0, so add 1 to get the line number a reader would see.
    for (index, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((index + 1, line));
        }
    }

    results
}
```

The return type is a vector of pairs. Each pair holds the line number, and the line itself, which still borrows from `contents`, so the lifetime `'a` is the same
as before.

### The test

A test checks the numbers, not only the text:

```rust,ignore
#[test]
fn line_numbers_are_one_based() {
    let query = "duct";
    let contents = "\
duct first
Pick three.
safe, fast, productive.
Rust:";

    assert_eq!(
        vec![(1, "duct first"), (3, "safe, fast, productive.")],
        search_with_line_numbers(query, contents)
    );
}
```

The first match is on line 1, and the second is on line 3, and neither is at the end. The test expects those numbers, which shows that the numbers start at 1.

### The run function

Change the printing loop in `run` to use the new function:

```rust,ignore
for (number, line) in search_with_line_numbers(&config.query, &contents) {
    println!("{number}: {line}");
}
```

With `cargo run -- body poem.txt`, the output is the three lines shown in the exercise.

### The extension

Both searches have the same loop, with only the line test and the value pushed being different. A closure or a generic helper could hold the shared loop, which
Chapter 13 covers. For now, two short loops are fine.

---

*This walkthrough accompanies the exercise in Chapter 12 of the course. It's not from the book.*
