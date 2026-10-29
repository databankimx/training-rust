## Objects and Encapsulation

The book's first section asks what object-oriented means, and notes that there's no agreed answer. It uses the definition from the *Design Patterns* book, in which objects
package data with the procedures that operate on that data, which are called methods. Rust's structs and enums hold data, and `impl` blocks give them methods, so Rust meets that
definition.

The second idea is *encapsulation*. Code that uses an object should reach it only through its public API, and never change its internals directly. That lets the internals change later
without changing the code that uses them. Rust does this with `pub`, which Chapter 7 covered: everything is private by default.

The sample is a collection that keeps its average up to date:

```rust,ignore
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

The fields are private, so the list can change only through `add` and `remove`, and each of those updates the average. A caller can read the average but can't set it, and can't change
the list without updating the average:

```rust,ignore
pub fn add(&mut self, value: i32) {
    self.list.push(value);
    self.update_average();
}
```

`main` adds three values, and then removes the last one. The output is:

```
average: 5.0
removed Some(9), average is now 3.0
```

Because the list is private, the internal representation could change later. The list could become a `HashSet`, for example, and `main` wouldn't need to change, as long as the public methods
kept their signatures.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest reading the list field directly from `main`. The compiler rejects it, because the field is private to the `averaged_collection` crate.

---

*For more detail, see section 18.1 "Characteristics of Object-Oriented Languages" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-01-what-is-oo.html).*
