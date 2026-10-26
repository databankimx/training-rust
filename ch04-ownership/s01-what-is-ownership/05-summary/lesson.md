## Summary: Ownership in Brief

You've now seen the core of ownership. Before moving on, here is the whole idea in one place, with each part
linked to the step where we covered it.

---

### Safety comes first

Rust's rules exist to keep programs *safe*, which means free of undefined behavior. Some mistakes, such as using
a variable before it has a value, are rejected at compile time, so they never reach a running program.
*(See the step on memory safety.)*

---

### Values live in frames and on the heap

Each function call has a stack frame that holds its variables. Data whose size is known and simple, such as a
number, is copied cheaply. Data that needs to live longer than a call, or that can grow, goes on the heap, and a
variable on the stack holds a pointer to it. *(See the stack-and-heap step.)*

---

### Every heap value has exactly one owner

The owner of a box or a collection is responsible for freeing its heap memory when the owner goes out of scope.
Rust frees it automatically, and nothing else may free it. *(See the deallocation step.)*

---

### Assigning a heap value moves it

Assigning a heap value to another variable, or passing it into a function, moves ownership. The old variable can no
longer be used, which prevents two owners from freeing the same memory. Rust enforces this, and tells you exactly
where the move happened. Cloning makes an independent copy when you need one. Collections such as `String` and
`Vec` follow the same rule. *(See the moves step.)*

---

### Ownership is a discipline for safety

Ownership isn't a set of arbitrary restrictions. Each rule exists to avoid a specific kind of undefined behavior.
When the compiler rejects a program, the error is usually pointing at one of those behaviors, so the rules are
worth understanding rather than memorizing.

---

*The summary is adapted from the Chapter 4 summary in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).*
