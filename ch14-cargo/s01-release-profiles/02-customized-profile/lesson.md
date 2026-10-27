## Overriding a Profile Setting

The defaults suit most projects, but a profile can be changed in `Cargo.toml`. A `[profile.dev]` section overrides the settings it names, and leaves the rest of the
`dev` profile as it was:

```toml
[profile.dev]
debug-assertions = false
```

`debug-assertions` controls the `debug_assertions` configuration that `cfg!` reads. With it turned off, the `dev` build has the same check as a release build, so the
program prints the release message:

```
This is a release build, made with the release profile
```

The program was built with `cargo run`, which uses the `dev` profile, so the only difference is the setting in `Cargo.toml`.

You can override `opt-level` in the same way. For example, `opt-level = 1` gives the `dev` profile a few optimizations, without the full cost of a release build. The
book's full list of settings is in Cargo's documentation.

---

### Try it in the runner

Press **Run**. The output should be the release message.

The comments in `src/main.rs` suggest removing the `[profile.dev]` section, then running again. The debug message comes back, because the default is restored.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
When you run `cargo build` with no additional flags, which profile is used by default?

◉ `dev`
→ Yes. `cargo build` uses the `dev` profile, which compiles quickly and includes debugging information.
○ `release`
→ Not quite. `release` is used by `cargo build --release`, which compiles more slowly to make the program faster.
○ `build`
→ Not quite. There's no profile called `build`. The names come from the profiles, not from the command.
````

---

*For more detail, see section 14.1 "Customizing Builds with Release Profiles" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-01-release-profiles.html).*
