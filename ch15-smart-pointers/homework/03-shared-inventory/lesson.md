## Homework: A Shared Inventory

**The problem:** A warehouse and a store both need to use one inventory list. Share it between the two of them with `Rc<RefCell<Vec<String>>>`, and keep a `Weak`
handle, so you can check whether the inventory still exists after its owners have finished with it.

Create a project in VS Code with `cargo new inventory`. In `src/main.rs`:

* Create one shared inventory, with `Rc::new(RefCell::new(Vec::new()))`.
* Make two more owners with `Rc::clone`, one called `warehouse` and one called `store`.
* Make a `Weak` handle to the same inventory with `Rc::downgrade`.
* Have the warehouse add `apples`, and the store add `pears`, using `borrow_mut`.
* Print the items, using `borrow`, and print the strong count.
* Drop the owners one at a time, and print the strong count after each one. Then drop the last owner, and use the `Weak` handle to print whether the inventory still exists.

Write two tests in a `tests` module. One checks that `upgrade` returns `Some` while an owner exists. The other checks that it returns `None` after the last owner is dropped.

---

### Hints

* `Rc::strong_count` counts the owners, including the one you're asking through.
* `borrow_mut().push(...)` adds an item. The `RefCell` allows it, even though the `Rc` is shared.
* `Weak::upgrade` returns `Option<Rc<...>>`, so `match` on it, or use `is_some` and `is_none`.
* Name the type of an empty vector, such as `Vec::<String>::new()`, when the compiler can't tell what it holds.

---

### Check your work

Your program should print:

```
items: ["apples", "pears"]
owners: 3
owners after dropping shared and warehouse: 1
inventory is gone
```

Check the owner count by hand. There are three owners at first, and dropping two leaves one.

---

### Extensions

* Add a second `Weak` handle, and check it after the last owner is dropped. Does it behave the same way?
* Show that a `Weak` handle doesn't keep the inventory alive: create one, then drop every `Rc`, and check that `strong_count` is 0 before you call `upgrade`.

---

*This exercise is not from the book. It practices `Rc`, `RefCell`, and `Weak` from Chapter 15.*
