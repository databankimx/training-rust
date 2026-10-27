## 14.1 Customizing Builds with Release Profiles

Cargo's build profiles control how code is compiled. The `dev` profile favors quick compiles, and the `release` profile favors fast programs. This sub-section shows which
profile a build used, and how a `[profile.dev]` section in `Cargo.toml` changes one setting.

* [The default profile](#goto/01-default-profile): `cfg!(debug_assertions)`, and the difference between `cargo build` and `cargo build --release`.
* [Overriding a profile setting](#goto/02-customized-profile): changing one `dev` setting in `Cargo.toml`, and what the program reports.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
