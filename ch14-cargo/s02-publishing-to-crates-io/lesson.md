## 14.2 Publishing a Crate to Crates.io

Once a library is ready, other people can use it through crates.io. This sub-section covers the parts you can try locally: documentation comments that double as tests,
and a public API that's convenient to use. It ends with the publishing steps, which are a terminal task.

* [Doc comments and doc tests](#goto/01-doc-comments-and-doc-tests): `///` comments, the documentation examples that `cargo test` runs.
* [Exporting a public API with pub use](#goto/02-public-api-with-pub-use): re-exporting items at a convenient path.
* [Publishing](#goto/03-publishing): the account, the metadata, `cargo publish`, and yanking.
* [Summary](#goto/04-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
