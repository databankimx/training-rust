## 4.1 What Is Ownership

Ownership starts with memory. This sub-chapter explains why Rust needs a memory model at all, and how it keeps the model
safe. You'll see how values live on the stack and the heap, how boxes own heap data, and how ownership moves from one variable to
another.

* [Memory safety](#goto/01-memory-safety): what "safe" means in Rust, and why undefined behavior matters.
* [The stack and the heap](#goto/02-stack-and-heap): where values live, and how frames and boxes work.
* [Deallocation](#goto/03-deallocation): when heap memory is freed, and why only one owner may free it.
* [Moves](#goto/04-moves): what happens when ownership is transferred, and how to clone when you need a copy.
* [Summary](#goto/05-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
