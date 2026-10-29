## Solution: A Documented Library

This walkthrough explains the example solution in `src/lib.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the unit tests and the doc tests. The output is:

```
words: 4
average length: 4.00
longest: Some("brown")
```

### How it works

Each public function has a summary line, and a documentation comment is written in Markdown. The `# Examples` heading starts a section that shows how to call the
function. Each code block there is a doc test, so `cargo test` compiles and runs it.

`average_word_length` has a `# Panics` section. It says when the function panics, so a caller knows to check the text first. The example in that section uses a
fence marked `should_panic`, so the doc test passes only when the call panics. Without that marking, the example would fail, because the call would panic.

`longest_word` returns the last of several equally long words, and the docs say so. `max_by_key` behaves that way in the standard library. Writing the rule into the
docs means a caller doesn't have to read the code to learn it.

The `//!` comment at the top documents the crate as a whole. It appears on the front page of the generated documentation.

### The extensions

With `#![deny(missing_docs)]` at the top of `lib.rs`, the compiler fails the build for any public item without a doc comment. It reports the first one it finds, which is
the item nearest the start of the file.

A `# Errors` section works like `# Panics`, for functions that return a `Result`. It describes the conditions under which an `Err` is returned, so a caller can decide
how to handle each one.

---

*This walkthrough accompanies the exercise in Chapter 14 of the course. It's not from the book.*
