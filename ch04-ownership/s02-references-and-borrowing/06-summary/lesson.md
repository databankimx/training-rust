## Summary: References and Borrowing

References let you read and write data without taking ownership of it. They're created with borrows, `&` and `&mut`,
and they're used with dereferences, `*`, which Rust often inserts for you.

References are powerful, and they can be misused. Rust's borrow checker uses a system of permissions to keep them safe.
The core of it is in these points:

- Every variable has read, write, and own permissions on its data, though write is only available with `mut`.
- Creating a reference transfers permissions from the borrowed place to the reference. A shared reference takes away
  write and own. A mutable reference takes away everything.
- Permissions are returned once the reference's lifetime has ended, which is the last place it is used.
- Data must outlive all references that point to it. Inside a function, the compiler can see this from permissions alone.
  Across a function boundary, it uses the flow permission, and lifetime parameters in the signature.

Underneath all of this is the pointer safety principle: data should never be aliased and mutated at the same time. Owned
data can only be reached through its owner. References can create temporary aliases, and the borrow checker makes sure
those aliases are not used while the data is being changed.

This section probably felt like it described more of what Rust *cannot* do than what it *can* do. That's intentional.
Rust lets you use pointers without a garbage collector, while avoiding undefined behavior. Understanding these rules now
will help you avoid frustration with the compiler later, and it will help you read the error messages it gives you.

---

*The summary is adapted from the end of section 4.2 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).*
