## Summary: Workspaces

A workspace groups related packages, so they share one `Cargo.lock` and one `target` folder.

* **Layout:** the top-level `Cargo.toml` has a `[workspace]` section, and lists its packages as members. *(See the creating a workspace step.)*
* **Dependencies:** a package names the other packages it uses, with a path dependency. External crates are unified across the workspace, when their versions are
  compatible. *(See the workspace dependencies step.)*
* **Commands:** `-p` picks one package, for `cargo run`, `cargo test`, and `cargo publish`. *(See the workspace dependencies step.)*

As a project grows, a workspace keeps each part small and easy to follow, and it makes it easier to change several crates at once.

---

*The summary is written for this course, and draws on the summary of section 14.3 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).*
