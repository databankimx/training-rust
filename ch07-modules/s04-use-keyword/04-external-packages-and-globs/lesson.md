## External Packages, Nested Paths, and Globs

Three more ways to write `use` make larger programs easier to read: external packages, nested paths, and the glob operator.

---

### External packages

To use a package from crates.io, you list it in `Cargo.toml`, and then bring its items into scope with `use`. In Chapter 2, the guessing game
did this with the `rand` crate.

The standard library, `std`, is also an external crate, but it ships with Rust. So you don't list it in `Cargo.toml`. You only write `use`
for the items you need, and the path starts with `std`:

```rust,ignore
use std::collections::HashMap;
```

---

### Nested paths

When several items share a prefix, you can bring them all into scope in one line. The common part of the path comes first, followed by `::`
and a list of the differing parts in curly brackets:

```rust,ignore
use std::collections::{BTreeSet, HashMap};
```

The sample uses this form. It makes a `HashMap` and a `BTreeSet`, and prints one value from each:

```
Some(10)
{"Corro", "Ferris"}
```

The `HashMap` stores a score for "Ferris", and `get` returns it as an `Option`. The `BTreeSet` keeps its names in sorted order, so "Corro" comes
before "Ferris" when printed.

A nested path can appear at any level. For example, `use std::io::{self, Write};` brings in `std::io` itself, and the `Write` trait, in one
line. The `self` stands for the path it's written in.

---

### Glob operator

The `*` glob operator brings every public item from a path into scope:

```rust,ignore
use std::collections::*;
```

This is convenient, but it's risky. It becomes hard to tell where a name came from, and if a dependency adds a new public item with the same name
as one of yours, your code can stop compiling after an update. The glob is most often used in tests, and as part of the prelude pattern.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Some(10)
{"Corro", "Ferris"}
```

The comments in `src/main.rs` suggest adding a glob import of `std::collections`. Nothing changes in this program, because the names it uses are
already in scope. The risk is in larger programs, where a glob can silently bring in a name that clashes with one of yours.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider this module and `use` statement:

```rust
pub mod parent {
  pub fn a() {}
  fn b() {}
  pub mod child {
    pub fn c() {}
  }
}

fn main() {
  use parent::{*, child as alias};
  // ...
}
```

Inside `main`, how many paths can refer to `a`, `b`, or `c`? Paths that use `self`, `super`, or `crate` are not counted.

○ 3
→ Not quite. There are more paths to `a` and `c` than that, including the ones the glob brings in.
○ 4
→ Not quite. One path to `c` is missing from that count. The glob brings in `child` as well as the alias.
◉ 5
→ Yes. There are two paths to `a`, `parent::a` and `a`. There are no paths to `b`, because it is private. There are three paths to `c`: `parent::child::c`, `child::c`, and `alias::c`.
○ 6
→ Not quite. `b` is private to `parent`, so no path reaches it from `main`.
````

---

*For more detail, see section 7.4 "Bringing Paths into Scope with the use Keyword" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html).*
