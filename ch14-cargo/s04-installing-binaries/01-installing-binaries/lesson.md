## Installing Binaries with cargo install

`cargo install` downloads a crate from crates.io, builds it, and copies its program to a folder on your computer. It's a convenient way to get command line tools that
other Rust developers have shared. It isn't a replacement for your system's package manager.

Only crates with a *binary target* can be installed. A binary target is a runnable program, which a crate has when it includes a `src/main.rs` file. A library crate
has no binary target, because it's meant to be used by other code. A crate's README usually says whether it's a library, a binary, or both.

The installed programs go to the `bin` folder of Cargo's home folder. With a standard Rust installation, that's `$HOME/.cargo/bin`. Make sure that folder is on your
`PATH`, so the shell can find the programs.

---

### An example: ripgrep

Chapter 12 mentioned `ripgrep`, a fast search tool written in Rust. Install it with:

```console
cargo install ripgrep
```

Cargo downloads the crate, builds it in release mode, and copies the program to the `bin` folder. The last lines of the output name the installed program, which is `rg`:

```console
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

The program is named `rg`, not `ripgrep`, so that's the name you type. Once the folder is on your `PATH`, you can run `rg --help` and start searching files.

---

### Try it in a terminal

This step can't run in the runner, because installing changes your computer. In a terminal, try:

```console
cargo install ripgrep
rg --help
```

If `rg` isn't found, check that `$HOME/.cargo/bin` is on your `PATH`. On Windows, the folder is `%USERPROFILE%\.cargo\bin`.

---

*For more detail, see section 14.4 "Installing Binaries from Crates.io with cargo install" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html).*
