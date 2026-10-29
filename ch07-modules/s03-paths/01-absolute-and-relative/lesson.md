## Absolute and Relative Paths

To call a function, the compiler needs its path: the route through the module tree from somewhere it can be seen. A path works the same way
as a path in a filesystem. There are two kinds.

* An **absolute path** starts from the crate root. In the current crate, it begins with the keyword `crate`. For an external crate, it begins
  with the crate's name.
* A **relative path** starts from the current module. It begins with `self`, with `super`, or with a name defined in the current module.

Both kinds are followed by one or more names separated by `::`.

---

### The sample

The sample defines a module `front_of_house` with a child module `hosting`, and calls the same function two ways from `eat_at_restaurant`:

```rust,ignore
pub fn eat_at_restaurant() {
    // Absolute path: starts from the crate root.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path: starts from the current module, which is the crate root here.
    front_of_house::hosting::add_to_waitlist();
}
```

Both calls reach the same function, so both print a line. The output is:

```
Added to waitlist
Added to waitlist
```

`eat_at_restaurant` is defined at the crate root, the same level as `front_of_house`. That's why the relative path can start with `front_of_house`.

---

### Choosing between them

Which kind to use depends on what you're likely to move. If you move a module and the code that uses it together, a relative path stays valid. If you
move the caller separately, an absolute path stays valid. The book prefers absolute paths in general, because code definitions and their calls are
more often moved separately.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Added to waitlist
Added to waitlist
```

The comments in `src/main.rs` suggest changing the absolute path to start with `self::`. At the crate root, `self` refers to the same module as `crate`,
so the call still works.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
What is the keyword you use at the start of an absolute path to an item in the current crate?

○ `self`
→ Not quite. `self` starts a relative path, from the current module.
○ `super`
→ Not quite. `super` starts a relative path, from the parent module.
◉ `crate`
→ Yes. `crate` is the root of the current crate. For example, `crate::a::b` is the absolute path to `b` in module `a`.
○ `root`
→ Not quite. Rust has no `root` keyword. The crate root is written `crate`.
````

---

*For more detail, see section 7.3 "Paths for Referring to an Item in the Module Tree" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html).*
