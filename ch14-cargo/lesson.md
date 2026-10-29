## Chapter 14: More About Cargo and Crates.io

So far we've used the basic features of Cargo: building, running, and testing. Cargo can do more, and this chapter covers the advanced features that help a project grow,
share its code, and install tools.

The chapter is split into sub-sections:

* [14.1 Customizing builds with release profiles](#goto/s01-release-profiles): the `dev` and `release` profiles, and overriding a setting in `Cargo.toml`.
* [14.2 Publishing a crate to crates.io](#goto/s02-publishing-to-crates-io): documentation comments and doc tests, a convenient public API with `pub use`, and publishing.
* [14.3 Cargo workspaces](#goto/s03-workspaces): a workspace with a binary and two libraries, and how its packages share dependencies.
* [14.4 Installing binaries](#goto/s04-installing-binaries): `cargo install`, the `bin` folder, and installing a command line tool.
* [14.5 Extending Cargo with custom commands](#goto/s05-extending-cargo): a program named `cargo-something` becomes `cargo something`.

---

## Summary

Cargo handles the tasks around the code as well as building it. Profiles tune how the code is compiled, crates.io shares it, workspaces organize related packages,
`cargo install` puts programs on your computer, and custom commands extend Cargo itself. Together they make Rust projects easier to build, share, and grow.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
