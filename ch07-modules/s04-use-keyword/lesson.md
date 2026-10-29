## 7.4 Bringing Paths into Scope with the use Keyword

Writing a full path for every reference is repetitive. The `use` keyword creates a shorter name for a path, in the scope where it's written.
This sub-chapter covers the basic form, the idioms and `as` for renaming, re-exporting with `pub use`, and how `use` works with external packages.

* [Bringing a module into scope](#goto/01-bringing-a-module-into-scope): creating a shortcut to a module, and what `use` does and doesn't change.
* [Idioms and as](#goto/02-idioms-and-as): how to write `use` by convention, and how to rename a name that clashes.
* [Re-exporting with pub use](#goto/03-re-exporting): making a shortcut available to code outside its scope.
* [External packages, nested paths, and globs](#goto/04-external-packages-and-globs): using `std` and crates, grouping imports, and the glob operator.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
