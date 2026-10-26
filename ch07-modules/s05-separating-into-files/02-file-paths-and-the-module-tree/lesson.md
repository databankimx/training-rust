## File Paths and the Module Tree

The last step split a module into files. This step covers the rules that connect the files to the module tree, and the older layout you'll still
see in some projects.

---

### One declaration per module

Each module is declared once, with `mod`, in the file that contains its parent. The compiler then looks for the module's body in a file with the
same name. Other files refer to the module by its path. They don't declare it again.

If you declare the same module in two places, the program won't compile, because the module would have two bodies.

---

### The module tree decides the file paths

The file path follows the module tree, not the other way round. A module declared in the crate root has its file in `src/`. A module declared
inside `front_of_house` has its file in `src/front_of_house/`. So the directory structure of `src` is the module tree, written out as folders.

Here is a package whose files show the tree:

```text
foobar
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── engine.rs
    └── engine/
        └── analysis.rs
```

* `lib.rs` declares `pub mod engine;`, so `engine` is a child of the crate root.
* `engine.rs` declares `mod analysis;` and re-exports its contents with `pub use analysis::*;`.
* `engine/analysis.rs` holds `pub fn run() {}`.

The tree that results is `crate`, then `engine`, then `run`, which is the function that `analysis` re-exports. A library user reaches it as
`foobar::engine::run`.

---

### The older layout

Rust also accepts a `mod.rs` file for a module that has children. Instead of `engine.rs` with an `engine/` folder, you can write `engine/mod.rs`. The
two forms behave the same, and you can't use both for the same module. Many people avoid the older layout now, because a project with many files named
`mod.rs` is hard to read when several are open at once.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Imagine a Rust package with the following directory structure:

```text
foobar
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── engine.rs
    └── engine/
        └── analysis.rs
```

The contents of the files are:

```rust
// engine/analysis.rs
pub fn run() {}
```

```rust
// engine.rs
mod analysis;
pub use analysis::*;
```

```rust
// lib.rs
pub mod engine;
```

Another developer uses the `foobar` library crate in a separate package, and wants to call `run`. Which path should they write?

◉ `foobar::engine::run`
→ Yes. `engine` is public in the crate root, and `engine.rs` re-exports `run`, so the path goes through `engine` directly.
○ `foobar::run`
→ Not quite. `run` is not defined in the crate root. It's re-exported by `engine`.
○ `foobar::engine::analysis::run`
→ Not quite. `analysis` is private to `engine`, so callers can't name it. The re-export makes `run` available through `engine` instead.
○ `engine::run`
→ Not quite. The path from another package has to start with the crate's name, `foobar`.
````

---

*For more detail, see section 7.5 "Separating Modules into Different Files" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html).*
