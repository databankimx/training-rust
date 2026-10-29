## 12.1 Accepting Command Line Arguments

The `minigrep` project starts with the arguments. A program that searches needs the text to search for, and the file to search in, and those come from the command
line. This sub-chapter covers reading them, and saving the two that matter.

* [Collecting the arguments](#goto/01-collecting-arguments): `std::env::args`, `collect`, and the program path.
* [Saving the argument values](#goto/02-saving-arguments): `query` and `file_path`, and what happens when an argument is missing.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
