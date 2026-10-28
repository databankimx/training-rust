## Solution: A Shared Inventory

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the two tests. The output is:

```
items: ["apples", "pears"]
owners: 3
owners after dropping shared and warehouse: 1
inventory is gone
```

### How it works

The inventory is an `Rc` around a `RefCell` around a `Vec`. The `Rc` lets three handles own it, and the `RefCell` lets the warehouse and the store add items through shared
references. Each `borrow_mut` is a short borrow that ends at the semicolon, so no two borrows overlap, and the runtime check never fails.

`Rc::strong_count` counts the owners. Three handles give 3. Dropping `shared` and `warehouse` leaves `store`, so the count is 1.

The `Weak` handle is made with `Rc::downgrade`. It doesn't add to the strong count, which is why the inventory can be freed when the last strong owner is dropped. After that,
`upgrade` returns `None`, and the program says so. This is the pattern Chapter 15.6 used to avoid cycles, applied to a plain shared value.

The tests cover both sides of `upgrade`. While an owner exists it returns `Some`, and after the last owner is dropped, it returns `None`.

### The extensions

A second `Weak` handle behaves the same way, because every `Weak` checks the same strong count. The count is in the shared allocation, not in each handle.

Dropping all the `Rc` handles leaves the strong count at 0. The value is freed at that point, even though a `Weak` still points at it. The `Weak` keeps only the memory for its bookkeeping
alive, which is why `upgrade` can still answer, and answers `None`, without reading freed data.

---

*This walkthrough accompanies the exercise in Chapter 15 of the course. It's not from the book.*
