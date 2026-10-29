## Summary: Method Syntax

Structs let you create custom types that are meaningful for your domain. They keep related data together, and name each piece, so the code says what it means.

Methods add behavior to a struct. They're defined in an `impl` block, and their first parameter is `self`, which stands for the instance they're called on.

* **Defining methods:** `impl` blocks hold the methods of a type. `&self` borrows the instance, which is what most methods need. *(See the defining methods step.)*
* **Methods with parameters:** after `self`, a method takes parameters like any function. *(See the methods with parameters step.)*
* **Methods and field names:** a method can share a field's name. Parentheses tell Rust which one you mean, and a method like that is often a getter. *(See the methods and field names step.)*
* **Associated functions:** functions in an `impl` block that don't take `self`, such as constructors. They're called with `::`. A type can have several `impl` blocks. *(See the associated functions step.)*
* **Method calls are function calls:** `r.area()` is shorthand for `Rectangle::area(&r)`, and Rust inserts the references for you. *(See the method calls step.)*
* **Methods and ownership:** `&self` reads, `&mut self` changes, and `self` takes ownership. Each one puts different demands on the caller's permissions. *(See the methods and ownership step.)*

Methods are the main way to organize the behavior of a type in Rust. When you write one, the signature tells the caller what it does to the instance,
and the borrow checker enforces that.

Enums are the next way to build custom types. They're covered in Chapter 6.

---

*The summary is adapted from the Summary of section 5.3 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).*
