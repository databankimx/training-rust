## Solution: A Workspace

This walkthrough shows one way to build the workspace. The steps are terminal tasks, so the solution is written out rather than run in the runner. Your files should match
these, apart from your name.

### The layout

After the steps are done, the folder should look like this:

```text
greetings
├── Cargo.lock
├── Cargo.toml
├── app
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── greeter
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── target
```

### The workspace file

The top-level `Cargo.toml` has no `[package]` section. Cargo adds the members to it as you create them:

```toml
[workspace]
resolver = "3"
members = ["app", "greeter"]
```

### The library

`greeter/src/lib.rs`:

```rust,ignore
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greet("Ada"), "Hello, Ada!");
    }
}
```

### The binary

`app/Cargo.toml` names the library as a path dependency:

```toml
[dependencies]
greeter = { path = "../greeter" }
```

`app/src/main.rs`:

```rust,ignore
use greeter::greet;

fn main() {
    println!("{}", greet("Ada"));
}
```

### Running it

From the top-level `greetings` folder:

```console
cargo run -p app
cargo test
```

`cargo run -p app` prints `Hello, Ada!`. `cargo test` runs the test in `greeter`, and shows a section for each package in the workspace.

### The extensions

A second library is added the same way, with `cargo new farewell --lib`. The `app` package depends on it with another path dependency line.

Removing `greeter` from `app/Cargo.toml` makes the `use greeter::greet;` line fail, with an error that says the crate can't be found. The library still exists in the
workspace, but `app` no longer declares it, and every package has to declare the dependencies it uses.

---

*This walkthrough accompanies the exercise in Chapter 14 of the course. It's not from the book.*
