## Summary: Fixing Ownership Errors

When the borrow checker rejects your code, the first question is always: **is my program actually unsafe?**

* If it is unsafe, you need to find the root cause. That usually means asking how long the data should live, and who
  should be in charge of freeing it. Then change the function's signature to match. Examples include returning
  the owned value, returning a `'static` literal, or letting the caller provide the space.
* If it is safe, the borrow checker has a limitation. Work around it by asking for only the permission you need,
  shortening a borrow so it doesn't overlap with a change, or using a standard library function such as
  `split_at_mut`, whose `unsafe` internals the borrow checker can't see into.

A few habits make these errors easier to handle:

* Ask for the *right* level of permission. Read-only inputs should be `&T`, not `&mut T` or `T`.
* Don't mutate an input the caller wouldn't expect to change.
* Shorten borrows. Copy out the value you need, such as a length, before you change the collection.
* Clone when you need an independent copy, but remember that cloning copies heap data.

These errors can feel frustrating, but each one points at a real question about who owns a value, and for how long.
Answering that question is the real skill.

---

*The summary is adapted from the section 4.3 summary in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-03-fixing-ownership-errors.html).*
