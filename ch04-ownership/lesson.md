## Chapter 4: Understanding Ownership

Ownership is the idea that makes Rust different from most languages. It's how Rust manages memory without a garbage
collector, and it's where most Rust programmers spend the most time learning. This chapter takes it slowly, and explains
the reasons behind each rule.

The chapter is split into sub-chapters, each covering one part of the story:

* [4.1 What is ownership](#goto/s01-what-is-ownership): memory safety, the stack and the heap, deallocation, and moves.
* [4.2 References and borrowing](#goto/s02-references-and-borrowing): references as pointers, aliasing, permissions, and mutable references.
* [4.3 Fixing ownership errors](#goto/s03-fixing-ownership-errors): how to respond when the borrow checker rejects your code.
* [4.4 Slices](#goto/s04-slices): a reference to part of a collection, and why it's useful.
* [4.5 Ownership recap](#goto/s05-ownership-recap): reviewing the chapter, and how it connects.

Take your time with this chapter. Each sub-chapter builds on the one before, and the homework at the end practises them together.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
