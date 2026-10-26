## Packages and Crates

Rust's module system has four parts: packages, crates, modules, and paths. This sub-chapter covers the first two, which are the
largest units of organization.

---

### Crates

A *crate* is the smallest amount of code the compiler considers at one time. Even a single file passed to `rustc` is a crate. Crates
can contain modules, which can live in other files that are compiled with the crate.

There are two kinds:

* A **binary crate** compiles to a program you can run. It has a `main` function. Every project you've built so far is a binary crate.
* A **library crate** has no `main`, and doesn't compile to a program. It provides code to be shared with other projects. Most people say "crate"
  to mean a library crate, and use it the way they'd use "library" in other languages.

The **crate root** is the source file the compiler starts from. It becomes the root module of the crate.

---

### Packages

A *package* bundles one or more crates, and has a `Cargo.toml` file that describes how to build them. Cargo itself is a package: it contains a
binary crate for the command-line tool, and a library crate that the tool depends on.

A package follows these rules:

* It can contain as many binary crates as you like.
* It can contain at most one library crate.
* It must contain at least one crate, a binary or a library.

Cargo finds the crate roots by convention. `src/main.rs` is the root of a binary crate with the same name as the package. `src/lib.rs` is the root
of a library crate with that name. Files in `src/bin/` each become a separate binary crate.

---

### The sample

The sample is a package named `greeter`. It has both kinds of crate: `src/lib.rs` is a library crate, and `src/main.rs` is a binary crate:

```rust,ignore
// src/lib.rs
pub fn greet(name: &str) -> String {
    format!("Hello, {name}, from the library crate!")
}
```

```rust,ignore
// src/main.rs
fn main() {
    println!("{}", greeter::greet("Ferris"));
}
```

The binary crate uses the library by the package's name, `greeter`. Its output is:

```
Hello, Ferris, from the library crate!
```

The `pub` keyword makes `greet` visible outside the library. You'll see why that matters in the sections on privacy.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Hello, Ferris, from the library crate!
```

The comments in `src/main.rs` suggest adding a second binary in `src/bin/other.rs`. The package then has three crates: the library, and two binaries.

---

*For more detail, see section 7.1 "Packages and Crates" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html).*
