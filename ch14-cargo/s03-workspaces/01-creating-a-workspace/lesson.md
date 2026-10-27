## Creating a Workspace

A workspace is a set of packages that share one `Cargo.lock` and one `target` folder. It suits a project that grows past a single library. The packages can then be
developed together, with each one small enough to understand on its own.

This walkthrough builds a workspace named `add`. It has a binary, `adder`, and two libraries, `add_one` and `add_two`. The binary depends on the libraries. This step
creates the workspace and its first package. It's a terminal task, so run these commands in a folder of your choice, such as one in VS Code.

```console
mkdir add
cd add
```

In the `add` folder, create a `Cargo.toml` that configures the whole workspace. It has no `[package]` section. It starts with a `[workspace]` section, and sets
`resolver` to `"3"`, which selects the latest version of Cargo's dependency resolver:

```toml
[workspace]
resolver = "3"
```

Then create the binary package inside the workspace:

```console
cargo new adder
```

Cargo adds `adder` to the `members` list of the workspace `Cargo.toml` automatically. Run `cargo build` in `add`, and the workspace has one `target` folder at the top:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The `adder` package has no `target` folder of its own. Even when you build from inside `adder`, the output goes to `add/target`. Sharing one folder means a crate that
another crate depends on isn't rebuilt for each package that uses it.

---

### Try it in VS Code

Work through the commands above, and check the folder layout against the tree. Then run `cargo build` from `add`, and look at where the output went.

---

*For more detail, see section 14.3 "Cargo Workspaces" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).*
