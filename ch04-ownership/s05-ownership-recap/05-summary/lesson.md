## Summary: Ownership

This chapter introduced ownership, which is Rust's way of managing memory without a garbage collector. It also covered
the concepts that go with it: references, slices, the stack and the heap, pointers, and undefined behaviour. Here's
how they fit together.

* **Ownership versus garbage collection:** a garbage collector frees memory at runtime, but it adds overhead and can make
  it hard to see which variables share data. Rust makes ownership and pointers explicit in each signature, so the cost and
  the connections are visible in the code. *(See the first step of this recap.)*
* **The runtime view:** variables live in stack frames, which are created and removed as functions are called. Boxes own
  data on the heap, and references point at data without owning it. *(See the runtime view step.)*
* **Permissions at compile time:** read, write, and own permissions decide what each operation may do. Moves remove all
  permissions from the moved variable, and borrows remove some from the borrowed data, for as long as the borrow is in use.
  *(See the permissions step.)*
* **Undefined behaviour:** use-after-free and double-free are the two kinds the rules are designed to prevent. The borrow
  checker always rejects programs with undefined behaviour, but it sometimes rejects safe programs too. *(See the undefined
  behaviour step.)*

When you're designing a function, the signature is where most of this is decided. Ask who owns each value, and how long
it must live. Ask which permissions the function really needs. Choose the signature that asks for the least, and the
borrow checker will be on your side.

As later chapters introduce structs, enums, and traits, each one will interact with ownership in specific ways. The ideas
in this chapter give you the vocabulary to reason about those interactions.

---

*The summary is adapted from the end of section 4.5 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-05-ownership-recap.html).*
