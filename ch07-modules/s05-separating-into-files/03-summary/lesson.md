## Summary: Managing Growing Projects

Rust lets you split a package into crates, and a crate into modules, so that items defined in one module can be used from another. You refer to
an item by an absolute or relative path, and you can bring a path into scope with `use`, so that later references are shorter. Module code is private
by default, and `pub` makes a definition public.

As a project grows, a module can move into its own file, and the module tree stays the same. The directory layout of `src` mirrors the tree, and paths in
the code don't change when a module moves into a file.

* **Packages and crates:** a package bundles crates, and a crate is the smallest unit the compiler considers at once. *(See the packages and crates step.)*
* **Modules:** modules group related code and control privacy, and they form a tree rooted at `crate`. *(See the defining modules step.)*
* **Paths:** an item is named by an absolute path from `crate`, or a relative path from the current module. `super` moves up one level, and `pub`
  makes an item reachable from outside its module. *(See the paths step.)*
* **use:** `use` creates a shorter name for a path, in the scope where it appears, and `pub use` makes that name available to others. *(See the use keyword step.)*
* **Files:** a module's body can live in its own file, found from the `mod` declaration. *(See this sub-chapter's steps.)*

The next chapter looks at collections in the standard library, which you can use in organized code like this.

---

*The summary is adapted from the Summary of Chapter 7 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html).*
