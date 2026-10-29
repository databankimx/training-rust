## Separating Modules into Different Files

As a module grows, its code gets harder to navigate in a single file. Rust lets you move a module's body into a file of its own, without
changing the module tree or any path that uses it.

---

### The layout

The sample has three source files. The layout of the files matches the module tree:

```text
src/
├── main.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

* `main.rs` is the crate root. It declares `front_of_house` with `mod front_of_house;`, and has no body for it.
* `front_of_house.rs` holds the body of `front_of_house`. It declares its child with `pub mod hosting;`.
* `front_of_house/hosting.rs` holds the body of `hosting`, with the function `add_to_waitlist`.

The file for each module sits in a place that follows its ancestors. `hosting` is a child of `front_of_house`, so its file goes in the
`front_of_house` directory.

---

### mod is a declaration, not an include

A `mod` line doesn't paste the file into the code. It tells the compiler that a module exists, and where to find its body. The compiler then
loads the file with the same name as the module. You only declare each module once, in the file that contains its parent. Everywhere else, you
refer to the module by its path, as you did in the previous sub-chapters.

The sample's `main` calls the function through its path, `front_of_house::hosting::add_to_waitlist()`. That path is the same as before the split.
The output is:

```
Added to waitlist
```

---

### Alternate file paths

Rust also accepts an older layout. Instead of `front_of_house.rs`, you can use `front_of_house/mod.rs`, and for `hosting`, you can use
`front_of_house/hosting/mod.rs`. Both forms work. Using both for the same module is an error, and mixing them across modules is allowed, but
confusing. Most projects use the layout in the sample, because many files named `mod.rs` are hard to tell apart in an editor.

---

### Try it in the runner

Use the drop-down above the editor to choose a file. `main.rs` is shown first. Then choose `front_of_house.rs` and `front_of_house/hosting.rs`.

Press **F5**. The Output pane should show:

```
Added to waitlist
```

The comments in `src/main.rs` suggest changing the message in `hosting.rs`. The runner can only edit and run `main.rs`, so make that change in your own editor, then run the step again. The other files are read-only in the runner.

---

*For more detail, see section 7.5 "Separating Modules into Different Files" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html).*
