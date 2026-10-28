## Chapter 19: Patterns and Matching

Patterns are a way to compare the shape of a value, and to bind names to its parts. They appear in `match`, `let`, `if let`, `while let`, `for`, and function parameters. This chapter
covers where patterns can be used, the difference between patterns that might fail and patterns that never do, and the syntax for writing them.

The chapter is split into sections, as the book has them:

* [19.1 All the places patterns can be used](#goto/s01-all-the-places-for-patterns): `match`, `let`, `if let`, `while let`, `for`, and function parameters.
* [19.2 Refutability](#goto/s02-refutability): patterns that might fail to match, and `let...else`.
* [19.3 Pattern syntax](#goto/s03-pattern-syntax): literals, ranges, `@` bindings, struct destructuring, and match guards.

---

## Summary

Patterns are part of Rust's everyday syntax. They take values apart in many places, and the compiler checks each one. Refutable patterns need a way to handle failure, and the syntax covers
literals, ranges, bindings, and guards, so a match can express most rules directly.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
