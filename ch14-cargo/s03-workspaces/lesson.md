## 14.3 Cargo Workspaces

A workspace manages several related packages together. This sub-section builds a small one, with a binary and two libraries, and shows how they share their
dependencies and their build output. The steps are terminal tasks, so they're done in a folder of your choice, in VS Code.

* [Creating a workspace](#goto/01-creating-a-workspace): the workspace `Cargo.toml`, the first package, and the shared `target` folder.
* [Dependencies in a workspace](#goto/02-workspace-dependencies): path dependencies, `cargo run -p`, and the quiz questions from the book.
* [Summary](#goto/03-summary): the main ideas of this sub-section.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
