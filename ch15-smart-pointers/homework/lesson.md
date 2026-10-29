## Chapter 15 Homework

This chapter covered smart pointers: `Box<T>`, `Deref` and `Drop`, `Rc<T>`, `RefCell<T>`, and `Weak<T>`. The exercises below practice three of them. Each one is a project
you write yourself in VS Code, starting with `cargo new`. Once you've tried one, reveal its example solution in the runner, and compare.

1. **A binary search tree.** Build a tree with `Option<Box<Node>>`, and add insert, search, and depth.
2. **A counted wrapper.** Write a smart pointer with `Deref` and `Drop`, which reports when it's created and dropped.
3. **A shared inventory.** Share one list between several owners, and use a `Weak` handle to see when the list is gone.

The exercises are not from the book.

---

*These exercises are not from the book. They practice the ideas in Chapter 15.*
