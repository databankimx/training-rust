## Publishing to Crates.io

Publishing puts a version of your crate on crates.io, where anyone can add it as a dependency. This step describes the process. It needs an account, and it changes a public
registry, so it's a terminal task, not something to run in the runner.

---

### Setting up

1. Create an account on [crates.io](https://crates.io/), by logging in with a GitHub account.
2. Get an API token from your account settings, and run `cargo login` in a terminal. Paste the token when Cargo asks for it.

The token is stored in a file in your Cargo home folder. It's a secret, so don't share it. If it leaks, revoke it on crates.io and create a new one.

---

### Metadata

Before publishing, the `[package]` section of `Cargo.toml` needs:

* A **name** that's unique on crates.io. Names are allocated first come, first served, so search before you choose one.
* A **description**, a sentence or two that appears in search results.
* A **license**, given as an SPDX identifier, such as `MIT`. Many Rust crates use `MIT OR Apache-2.0`, which lists two licenses.

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

If you run `cargo publish` without the description and license, Cargo warns about the missing fields, and crates.io rejects the upload.

---

### Publishing

Run `cargo publish` in the crate's folder. Cargo packages the crate, checks that it builds, and uploads it.

A publish is permanent. A version can't be overwritten or deleted. You can publish a new version, and you can have as many versions as you like.

To release a change, update `version` in `Cargo.toml`, using semantic versioning to decide the number, and run `cargo publish` again.

---

### Yanking a version

If a version is broken, you can stop new projects from depending on it. Run `cargo yank`, with the version:

```console
cargo yank --vers 1.0.1
```

Projects that already depend on that version keep working, because their `Cargo.lock` files still name it. Undo a yank with `--undo`. A yank doesn't delete any code, so
if you published a secret, reset the secret immediately.

---

*For more detail, see section 14.2 "Publishing a Crate to Crates.io" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html).*
