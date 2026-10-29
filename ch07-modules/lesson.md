## Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

As a project grows, its code needs structure. Rust gives you four tools for that: packages, crates, modules, and paths. Together they control
how code is grouped, what is visible to other code, and how names are brought into scope.

The chapter is split into sub-chapters:

* [7.1 Packages and crates](#goto/s01-packages-and-crates): the outer layers, and how Cargo finds them.
* [7.2 Defining modules](#goto/s02-defining-modules): declaring modules, using `pub` for privacy, and the module tree.
* [7.3 Paths for referring to an item](#goto/s03-paths): absolute and relative paths, `pub`, `super`, and public structs and enums.
* [7.4 Bringing paths into scope with use](#goto/s04-use-keyword): shortcuts with `use`, `as`, `pub use`, external packages, and globs.
* [7.5 Separating modules into different files](#goto/s05-separating-into-files): splitting a module into its own file, and how file paths follow the module tree.

That completes the chapter's material.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
