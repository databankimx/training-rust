## 6.3 Concise Control Flow with if let and let...else

`match` is the right choice when every case matters. This sub-chapter covers the shorter forms for when it doesn't: `if let` for one pattern,
`if let` with `else` for two branches, and `let...else` for keeping the main work on the top level of a function.

* [if let](#goto/01-if-let): handling one pattern, and ignoring the rest.
* [if let with else](#goto/02-if-let-else): handling a non-matching value in an `else` branch.
* [let...else](#goto/03-let-else): returning early, and keeping the rest of the function flat.
* [Summary](#goto/04-summary): choosing between `match`, `if let`, and `let...else`.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
