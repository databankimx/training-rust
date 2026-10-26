## Lessons From the Inventory

The two programs in this sub-chapter are small, and they look like everyday code. Each one fails for a reason that comes from ownership and
borrowing, and each fix comes from the same questions you've been asking throughout the course.

* **Returning a reference to a local value:** the referenced data is dropped when the function returns. Return an owned value instead, such as a
  `String`, when the function creates the data it returns.
* **Moving out of a reference:** a method such as `unwrap` takes ownership, and a reference can't give ownership away. Match on the reference, so
  the contents are borrowed rather than moved.

When a function fails to compile, ask two questions. What does the error say the function is trying to take or return? And which owner is supposed
to be responsible for the data? Usually the answers point to the fix.

The inventory also showed that the compiler can be right for the wrong-looking reason, and that the best fix often keeps the signature callers expect.
Changing a parameter from a reference to an owned value is almost always a more restrictive API than it needs to be.

---

*The summary is written for this course. The Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-04-inventory.html) has no separate summary.*
