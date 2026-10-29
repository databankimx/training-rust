## Chapter 12 Homework

This chapter built `minigrep`. The exercises below extend it. Start from your own `minigrep` project, as it stood at the end of section 12.5, which has a library
(`src/lib.rs`) with `Config`, `run`, `search`, and `search_case_insensitive`. Do the work in VS Code. The runner doesn't need to run these.

Keep the tests you already have, and add new ones. Run `cargo test` often, and `cargo run -- query file` to check the program by hand.

The exercises are not from the book.

1. **Line numbers.** Print each match with the number of its line.
2. **Validating the query.** Reject an empty query, with a helpful error and tests.
3. **Several files.** Search every file given after the query, and keep going when one file is missing.

---

*These exercises are not from the book. They extend the minigrep project from Chapter 12.*
