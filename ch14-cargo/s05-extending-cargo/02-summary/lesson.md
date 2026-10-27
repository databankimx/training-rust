## Summary: Extending Cargo

Cargo is more than a build tool. It tunes builds with profiles, shares code through crates.io, organizes related packages in workspaces, installs programs, and can be
extended with commands of your own.

* **Release profiles:** `dev` and `release` have different defaults, and `[profile.*]` sections change them. *(See the 14.1 steps.)*
* **Publishing:** documentation comments double as tests, `pub use` shapes the public API, and a published version is permanent. *(See the 14.2 steps.)*
* **Workspaces:** related packages share one `Cargo.lock` and one `target` folder. *(See the 14.3 steps.)*
* **Installing:** `cargo install` puts a program in the `bin` folder, where your `PATH` can find it. *(See the 14.4 step.)*
* **Custom commands:** a program named `cargo-something` becomes `cargo something`. *(See the custom command step.)*

Sharing code is part of what makes Rust useful. The standard library is small and stable, and crates can be shared and improved on their own schedule. If something
you write is useful to you, it may be useful to others too.

---

*The summary is written for this course, and draws on the summary of section 14.5 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-05-extending-cargo.html).*
