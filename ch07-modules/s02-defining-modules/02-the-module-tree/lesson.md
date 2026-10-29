## The Module Tree

Modules nest, so a crate's modules form a tree. Understanding that tree tells you where any item lives, and how to reach it.

---

### The crate is the root

Every crate has an implicit root module called `crate`. The crate root file, `src/main.rs` for a binary or `src/lib.rs` for a library, forms
this module. Everything else in the crate is inside it.

For the `front_of_house` example from the last step, the tree looks like this:

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

---

### Parents, children, and siblings

The tree gives the words for relationships between modules:

* `hosting` is nested inside `front_of_house`, so `hosting` is the *child* of `front_of_house`, and `front_of_house` is its *parent*.
* `hosting` and `serving` are defined in the same module, so they are *siblings*.

Every module's path starts at `crate`, or at a module that is above it. The next sub-chapter covers how to write those paths.

---

### Modules and the filesystem

The module tree looks like a directory tree, and that comparison is apt. Modules organize code the way directories organize files. Just as a file
needs a path to be found, a module needs a path too. Chapter 7.3 describes those paths, and Chapter 7.5 shows how modules map onto files.

---

### Why use modules?

The quiz asks which of several claims is not a benefit of modules. The answer is the one about runtime performance. Modules have no effect on how a
program runs. They're for organizing code at compile time, which is why they help people work on large codebases.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following is NOT a benefit of using modules?

○ Modules group related code so programmers can more easily work on a large codebase
→ Not quite. Grouping related code is one of the main reasons for modules.
○ Modules encapsulate implementation details that shouldn't be used by external clients
→ Not quite. Hiding details behind `pub` is a core benefit of modules.
○ Modules provide a scope to avoid naming conflicts across parts of a codebase
→ Not quite. Modules give each name its own scope, so two items can share a name in different modules.
◉ Modules boost the runtime performance of interdependent code within the same module
→ Yes. Modules are for organizing code, and they have no effect on runtime performance.
````

---

*For more detail, see section 7.2 "Defining Modules to Control Scope and Privacy" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html).*
