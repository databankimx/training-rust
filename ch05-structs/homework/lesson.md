## Chapter 5 Homework

This chapter introduced structs and methods: how to group related data, how methods borrow or take ownership of `self`,
and how the borrow checker applies to fields. The exercises below practice those ideas together.

The exercises are not from the book. Each one uses a different part of the chapter.

1. **Rectangles with methods.** Add methods to the `Rectangle` struct, including a constructor for squares.
2. **Shopping cart.** Write a small struct for an item, with methods that read and change it.
3. **Fix the ownership error.** A program calls a method that takes `self` through a reference. Fix it in two ways, and explain why one of them doesn't always work.

---

### How to use these exercises

Try each one yourself before you look at the solution. The lesson gives hints. If you get stuck, the solution walkthrough is behind
a **Show example (spoiler)** button, which shows the code and explains how it works.

When the compiler rejects your code, read the error from top to bottom. Ask which permission it says is missing, and which borrow is still in use.

---

*These exercises are not from the book. They're written to practice the ideas in Chapter 5.*
