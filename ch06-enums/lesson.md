## Chapter 6: Enums and Pattern Matching

Enums let a value be one of several possibilities, and each possibility can carry its own data. This chapter covers how to define
enums, and how to handle each possibility with `match`, `if let`, and related tools.

The standard library's `Option` type is an enum, and it's how Rust represents a value that may be absent, without `null`.

The chapter is split into sub-chapters:

* [6.1 Defining an enum](#goto/s01-defining-an-enum): defining enums, putting data in their variants, and `Option<T>`.
* [6.2 The match control flow construct](#goto/s02-match): handling each variant with `match`, including `Option`, exhaustiveness, and ownership.
* [6.3 Concise control flow with if let and let...else](#goto/s03-if-let): handling one pattern with `if let`, and returning early with `let...else`.
* [6.4 Ownership inventory](#goto/s04-ownership-inventory): two realistic ownership programs, and how to fix them.

That completes the chapter's material.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
