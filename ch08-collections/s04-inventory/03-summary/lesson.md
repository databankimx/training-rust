## Lessons From the Inventory

Both programs in this sub-chapter change a vector while looking at it. Each one fails for the same underlying reason: a borrow that's still in use
when the vector changes, or two mutable borrows that the compiler can't tell apart.

* **Changing a vector while iterating:** an iterator borrows the vector, so `remove` and `push` inside the loop conflict with it. Loop over the indexes
  instead, when the loop changes the vector. A range of numbers doesn't borrow anything.
* **Two mutable borrows of one vector:** indexing borrows the whole vector, so `&mut v[i]` and `&mut v[j]` at once conflict, even when `i` and `j` are
  different. Use a method built for this, such as `Vec::swap`, rather than reaching for `unsafe` code.

When the compiler rejects a loop, ask two questions. What is borrowed for the whole loop? And is the program actually unsafe, or is the compiler being
careful? Often the fix is to change how the loop refers to the data, rather than to change what the function does.

---

*The summary is written for this course. The Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-04-inventory.html) has no separate summary.*
