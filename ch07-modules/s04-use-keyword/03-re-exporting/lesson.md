## Re-exporting Names with pub use

When you bring a name into scope with `use`, the name is private to the scope that contains the `use`. Code in other modules can't see it
through that scope. `pub use` makes the name public, so it can be used from anywhere the name is visible. This is called *re-exporting*, because
you bring an item into scope, and make it available to others as well.

---

### The sample

The sample brings `hosting` into the crate root, and marks the `use` as `pub`:

```rust,ignore
pub use front_of_house::hosting;
```

A separate module, `customer`, then reaches `hosting` by the shorter name:

```rust,ignore
pub fn visit() {
    crate::hosting::add_to_waitlist();
}
```

Without the re-export, the path from `customer` would have to name `front_of_house` as well. The output is:

```
Added to waitlist
```

---

### Why it matters for libraries

Re-exporting matters most in a library. External code that depends on your crate can only use what your public API exposes. If a module is
nested deep inside private modules, a `pub use` at the top lets callers use a short path, even though your code is organized differently inside.

The book's restaurant example makes the point: staff think of the front of house and back of house, but customers don't. A library can be organized
for the people who maintain it, and expose a structure that suits the people who call it. A binary crate can't show that fully, because nothing
outside it calls it. The sample shows the mechanism, and the library case is where the benefit lies.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Added to waitlist
```

The comments in `src/main.rs` suggest changing `pub use` to `use`. In this binary, the program still compiles, because `customer` is inside the same
crate, and private names are visible to child modules. For a library used from another crate, the change would hide the name.

---

*For more detail, see section 7.4 "Bringing Paths into Scope with the use Keyword" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html).*
