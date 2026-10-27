## Customizing Builds with Release Profiles

Cargo has two main profiles. The `dev` profile is used by `cargo build`, and it's set up for development, so compiles are quick. The `release` profile is used by
`cargo build --release`, and it's set up for speed, so the program runs faster but takes longer to compile.

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

Each profile has defaults, which apply unless a `[profile.*]` section in `Cargo.toml` changes them. The most important setting is `opt-level`, which controls how
many optimizations are applied, from 0 to 3. `dev` uses 0, and `release` uses 3.

---

### Telling the two builds apart

This sample checks which profile it was built with:

```rust,ignore
if cfg!(debug_assertions) {
    println!("This is a debug build, made with the dev profile");
} else {
    println!("This is a release build, made with the release profile");
}
```

`cfg!` is decided at compile time, so the check adds no cost when the program runs. `debug_assertions` is turned on in the `dev` profile, and off in `release`. The
runner's **Run** button builds with the `dev` profile, so the output is:

```
This is a debug build, made with the dev profile
```

---

### Try it in the runner

Press **Run**. The output should match the line above.

The comments in `src/main.rs` suggest running `cargo build --release` in a terminal. The program then prints the release message, because it was compiled with
the `release` profile.

---

*For more detail, see section 14.1 "Customizing Builds with Release Profiles" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-01-release-profiles.html).*
