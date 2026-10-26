## Counting Crates

The last step introduced packages and crates. This short step checks how the two relate, and how to count the crates in a package
by looking at its files.

---

### The containment order

A package contains crates, and a crate contains modules. So from the outside in, the order is:

```text
package > crate > module
```

Each level holds the one below it. You'll meet modules in the next sub-chapter.

---

### Counting the crates in a package

Here is a package called `foobar`:

```text
foobar
├── Cargo.toml
├── build.rs
└── src/
    ├── main.rs
    ├── util.rs
    ├── lib.rs
    └── bin/
        └── alt.rs
```

Cargo treats each file that's a crate root as a crate:

* `src/main.rs` is a binary crate.
* `src/lib.rs` is a library crate.
* `src/bin/alt.rs` is another binary crate, because each file in `src/bin/` is one.

`src/util.rs` isn't a crate. It's not a special filename, so it's a file that a crate can load as a module. `build.rs` is a build script, which runs
before the crates are compiled, and isn't a crate either. So the package contains three crates.

---

### Try it

The sample from the last step has a library and a binary. If you add a file to `src/bin`, you'll have another binary crate, and that changes the
count. The count depends on which files are crate roots, not on how many files there are.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which is the correct order, where "A > B" means "A contains B"?

○ module > crate > package
→ Not quite. A module is inside a crate, not the other way around.
○ crate > package > module
→ Not quite. A package contains crates, so the package is the outer level.
◉ package > crate > module
→ Yes. A package contains crates, and a crate contains modules.
````

````quiz
Imagine you see a Rust package `foobar` with the following files:

```text
foobar
├── Cargo.toml
├── build.rs
└── src/
    ├── main.rs
    ├── util.rs
    ├── lib.rs
    └── bin/
        └── alt.rs
```

How many crates does this package contain?

○ 1
→ Not quite. There is a library crate, and at least two binary crates.
○ 2
→ Not quite. `main.rs`, `lib.rs`, and `bin/alt.rs` each produce a crate, so there are three.
◉ 3
→ Yes. `main.rs` and `bin/alt.rs` are binary crates, and `lib.rs` is a library crate. `util.rs` is a module, and `build.rs` is a build script.
○ 4
→ Not quite. `util.rs` isn't a crate, because it isn't a crate root that Cargo recognizes.
````

---

*For more detail, see section 7.1 "Packages and Crates" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html).*
