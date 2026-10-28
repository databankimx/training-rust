## Chapter 20: Advanced Features

This chapter covers features that most programs don't need, but which matter when they do. The book's sections are unsafe Rust, advanced traits, advanced types, advanced functions and closures,
and macros.

The chapter is split into sections, as the book has them:

* [20.1 Unsafe Rust](#goto/s01-unsafe-rust): raw pointers, a safe abstraction over unsafe code, mutable statics, and calling C.
* [20.2 Advanced traits](#goto/s02-advanced-traits): associated types, operator overloading, disambiguating names, supertraits, and the newtype pattern.
* [20.3 Advanced types](#goto/s03-advanced-types): newtypes for type safety, type aliases, the never type, and dynamically sized types.
* [20.4 Advanced functions and closures](#goto/s04-advanced-functions-and-closures): function pointers, named functions in adapters, and returning closures.
* [20.5 Macros](#goto/s05-macros): declarative macros, and procedural macros as a walkthrough.

---

## Summary

These features are for particular situations: unsafe code when the compiler can't prove correctness, traits and types that express more precise relationships, function values that
fit many callers, and macros that remove repetition or generate code. Each one has a cost, and knowing when to reach for it is most of the skill.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
