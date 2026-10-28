## Solution: A Binary Search Tree

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the tests. The output is:

```
contains 4: true
contains 7: false
depth: 3
```

### How it works

Each `Node` owns its children through `Option<Box<Node>>`. The box gives the node a known size, which is what makes the recursive type possible, as Chapter 15.1 showed.
When a node is dropped, its children are dropped with it, so the whole tree is freed without any extra code.

`insert` delegates to `insert_into`, which takes a mutable reference to a slot. The slot is either `None`, in which case it's filled with a new node, or `Some` node, in
which case the function recurses into the left or right side. Equal values do nothing, so duplicates are ignored. Passing `&mut node.left` gives the recursive call a
mutable reference to one field, and the borrow checker accepts that, because the parent isn't used again until the call returns.

`contains` is a loop that holds a shared reference to the current slot. Shared references can be copied, so reassigning `current` is simple.

`depth` is recursive. An empty slot has depth 0, and a node has one more than the deeper of its two children. The helper is a function inside `depth`, so it stays private to
the method.

### The extensions

Removing a node with two children is the hard case. Its place must be taken by either the largest value in its left subtree, or the smallest in its right subtree. Taking
the value from that node, and removing it from its subtree, keeps the ordering correct.

`contains` can be written with a loop, and the recursive version of `depth` is still fine for small trees. A very deep tree could overflow the stack with recursion, and a loop
doesn't add frames. For a balanced tree, the depth is small, so the risk is mostly for trees built from sorted input.

---

*This walkthrough accompanies the exercise in Chapter 15 of the course. It's not from the book.*
