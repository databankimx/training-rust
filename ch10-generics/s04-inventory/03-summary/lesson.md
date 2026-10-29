## Lessons From the Inventory

Both programs look like everyday code, and each one fails because of a borrow or a reference that the compiler reads more strictly than the programmer meant.

* **Sorting a borrowed slice:** a function that only reads its input shouldn't mutate it. Sort a `Vec` of references instead, so the data itself never moves.
* **Calling a helper that borrows all of `self`:** the borrow from the helper covers the whole struct. If the helper returns something you can copy, copy it first, and the
  loop can then borrow only the field it changes.

When the compiler rejects a function, ask what the function is meant to do, and which borrow is still alive when it does it. The fix that keeps the intent of the
function, and borrows only what it needs, is usually the right one.

---

*The summary is written for this course. The Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-04-inventory.html) has no separate summary.*
