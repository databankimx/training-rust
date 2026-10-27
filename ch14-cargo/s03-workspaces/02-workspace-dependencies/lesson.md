## Dependencies in a Workspace

The workspace from the last step gets two libraries. `add_one` provides an `add_one` function, and `adder` depends on it. Cargo doesn't assume packages in a workspace depend on
each other, so each dependency is declared.

Create the library from inside the `add` folder:

```console
cargo new add_one --lib
```

Cargo adds `add_one` to the `members` list. In `add_one/src/lib.rs`, write the function:

```rust,ignore
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Then tell `adder` where to find `add_one`, with a path dependency in `adder/Cargo.toml`:

```toml
[dependencies]
add_one = { path = "../add_one" }
```

In `adder/src/main.rs`, call the library:

```rust,ignore
use add_one::add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one(num));
}
```

Run the binary from the `add` folder, with `-p` to name the package:

```console
cargo run -p adder
```

The output is:

```
Hello, world! 10 plus one is 11!
```

`-p` matters in a workspace. Without it, Cargo doesn't know which package's binary to run, and it stops with an error.

---

### One Cargo.lock for every package

The workspace has one `Cargo.lock`, at the top level. If two packages use the same external crate, such as `rand`, they share one version of it. Compatible versions are
unified automatically. Each package still needs to list the crate in its own `Cargo.toml`. If `adder` uses `rand` without listing it, the build fails with an unresolved
import, even though `add_one` lists it.

Cargo applies Semantic Versioning when it chooses versions. Versions such as `0.8.1` and `0.8.0` are compatible, so they're unified. Versions such as `0.7` and `0.8` are not,
so each one is used separately.

---

### Tests in a workspace

Running `cargo test` from the top-level folder runs the tests in every package. Add a test to `add_one`, and `cargo test -p add_one` runs just that package's tests.

---

### Try it in VS Code

Build the libraries, run `cargo run -p adder`, and then remove the `add_one` dependency from `adder/Cargo.toml`. What error does Cargo report, and why?

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider a workspace with this structure:

```
├── Cargo.lock
├── Cargo.toml
└── crates
    ├── a
    |   ├── lib.rs
    |   └── Cargo.toml
    └── b
        ├── lib.rs
        └── Cargo.toml
```

The file `crates/b/lib.rs` contains `pub fn f() {}`. The file `crates/a/Cargo.toml` has `b = {path = "../b"}` under `[dependencies]`.

Which is the correct path to refer to `f` inside `a/lib.rs`?

○ `crates::b::f`
○ `workspace::b::f`
◉ `b::f`
○ `super::b::f`
→ Yes. From `a`, `b` is an external crate like any other, so you use its name directly, with no prefix. `super` refers to the parent module, not another crate.
````

````quiz
Consider a workspace with three packages, `a`, `b`, and `c`. Their dependencies are:

```toml
# a/Cargo.toml
[dependencies]
rand = "0.8.4"

# b/Cargo.toml
[dependencies]
rand = "0.7.3"

# c/Cargo.toml
[dependencies]
rand = "0.8.5"
```

How many different versions of `rand` are used when all the packages are compiled?

○ 1
◉ 2
○ 3
○ 4
→ Yes. `a` and `c` share the latest `0.8` release, because `0.8` versions are compatible under Semantic Versioning. `b` uses `0.7`, which is incompatible with `0.8`, so it gets its own copy.
````

---

*For more detail, see section 14.3 "Cargo Workspaces" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).*
