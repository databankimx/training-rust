## Exporting a Convenient Public API with pub use

The structure that's convenient while you develop a crate isn't always the one users want. A crate might keep its types in nested modules, which are good for
organizing the code, but make it hard for a user to find a type. A user would have to write `use art::kinds::PrimaryColor;`, and learn the module layout to do it.

The fix doesn't require reorganizing the code. A `pub use` statement re-exports an item, which makes it public somewhere else. The item keeps its place in the
modules, and gains a convenient path at the top level:

```rust,ignore
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

Users can now write `use art::{PrimaryColor, mix};`, and the modules are no longer part of how the crate is used. The `kinds` and `utils` modules are still public, so the
long paths still work. This matters in a large crate, where the layout can be deep.

The same idea works with dependencies: a crate can re-export a type from one of its dependencies, so its own public API includes that type.

---

### Try it in the runner

Press **Run** for the program's output, which is `Red and yellow make Orange`. Then press **Test** for the unit tests in `src/lib.rs`.

The comments in `src/main.rs` suggest importing `art::kinds::PrimaryColor` through the long path. Both paths work, because the module stays public. The re-export is a
choice about what users see, not a restriction.

---

*For more detail, see section 14.2 "Publishing a Crate to Crates.io" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html).*
