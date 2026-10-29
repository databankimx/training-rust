## Homework: A Binary Search Tree

**The problem:** Build a binary search tree of integers. Each node holds a value, and an optional left and right child. Smaller values go left, and larger or equal values go
right. Use `Option<Box<Node>>` for the children, so each node owns the nodes below it.

Create a project in VS Code with `cargo new bst`. In `src/main.rs`, write:

* A struct `Node` with a `value: i32`, a `left: Option<Box<Node>>`, and a `right: Option<Box<Node>>`.
* A struct `Tree` with a `root: Option<Box<Node>>`, and a `new` method that makes an empty tree.
* `insert(&mut self, value: i32)`, which adds a value. A duplicate is ignored.
* `contains(&self, value: i32) -> bool`, which says whether the tree holds a value.
* `depth(&self) -> usize`, which returns the number of levels. An empty tree has depth 0, and a single node has depth 1.

Write tests in a `tests` module, and a `main` that inserts 5, 3, 8, 1, 4, and then prints whether the tree contains 4 and 7, and its depth.

---

### Hints

* `insert` on a node walks down with a `match` on the child. If the child is `None`, put a new `Box::new(Node { ... })` there. Otherwise, recurse into the child.
* A `&mut` reference to an `Option<Box<Node>>` is the easiest way to write insert. `as_mut()` and `as_deref()` help with the borrowing.
* `depth` can be recursive: one plus the larger of the two children's depths, with `Option::map_or` for the `None` case.

---

### Check your work

Your `main` should print:

```
contains 4: true
contains 7: false
depth: 3
```

The tree after inserting 5, 3, 8, 1, 4 has 5 at the root, 3 and 8 below it, and 1 and 4 below 3. The longest path, 5, 3, 1, has three nodes.

---

### Extensions

* Add `remove`. What's the hard case, and how do you handle it?
* Make `contains` iterative, with a `while let` loop, instead of recursive. Why might that matter for a very deep tree?

---

*This exercise is not from the book. It practices `Box<T>` and recursive types from Chapter 15.*
