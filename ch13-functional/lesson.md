## Chapter 13: Functional Language Features

Rust has borrowed ideas from functional languages. Closures are functions that can capture their surroundings, and iterators let you process a sequence of values with
a chain of operations. This chapter covers both, uses them to improve `minigrep`, and checks that they cost nothing at runtime.

The chapter is split into sub-chapters:

* [13.1 Closures](#goto/s01-closures): capturing the environment, closure types, capture modes, and the `Fn` traits.
* [13.2 Iterators](#goto/s02-iterators): creating and using iterators, consuming and adapting methods, and closures in `filter`.
* [13.3 Improving the I/O project](#goto/s03-improving-the-io-project): `Config` with iterators, `search` with adapters, and returning an iterator.
* [13.4 Performance](#goto/s04-performance): loops and iterators, timed, and why iterators are zero-cost.

---

## Summary

Closures let a function's body use the values around it, and the three `Fn` traits describe how a closure uses what it captures. Iterators describe a sequence of steps,
run only when a consuming method asks for the results. Together they let code state its intent at a high level, while the compiler produces code as fast as a loop
written by hand.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
