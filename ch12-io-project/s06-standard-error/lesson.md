## 12.6 Writing Errors to Standard Error

Good command line programs keep their results and their errors apart, so a user can redirect the results to a file, and still see any error on screen. This sub-chapter
explains the two streams, and shows that `minigrep` writes its errors with `eprintln!`.

* [stdout and stderr](#goto/01-stdout-and-stderr): `println!` and `eprintln!`, and redirecting output in a terminal.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
