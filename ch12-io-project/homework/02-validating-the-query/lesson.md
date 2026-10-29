## Homework: Validating the Query

**The problem:** An empty query, such as `cargo run -- "" poem.txt`, matches every line, because every string contains the empty string. That's almost never what
a user wants. Make `Config::build` reject an empty query, with a clear message.

Work in your own `minigrep` project, from VS Code. Then:

* Change `Config::build` so that an empty query returns an error, with the message `query must not be empty`.
* Keep the existing checks. A missing file path still returns `not enough arguments`.
* Write tests for `Config::build` in a `tests` module inside `src/lib.rs`. They should check three cases: a valid query and file, an empty query, and too few
  arguments. Build the argument vector in each test, as the program does, with the program name first.

Think about what the tests need to compare. `Result<Config, &str>` doesn't implement `PartialEq` for `Config`, so `assert_eq!` on the whole value won't compile.
Match on the result, or check its fields.

---

### Hints

* The program name is the first item of the argument vector, so `vec!["minigrep", "", "poem.txt"]` is an empty-query case.
* `query.is_empty()` checks for an empty string. Decide whether a query of spaces should also be rejected, and say why in a comment.
* `match` on the result of `Config::build`. Inside an `Ok` arm, check the fields with `assert_eq!`. Inside an `Err` arm, check the message.

---

### Check your work

Run `cargo run -- "" poem.txt`. The program should print `Problem parsing arguments: query must not be empty` on standard error, and exit with code 1.
`cargo test` should show three new tests, all passing.

---

### Extensions

* Reject a file path that is empty too. Should that check be in `Config::build`, or in `run`? Why?
* Turn the three error messages into an enum, so the caller can match on the kind of error without comparing text.

---

*This exercise is not from the book. It extends the minigrep project from Chapter 12.*
