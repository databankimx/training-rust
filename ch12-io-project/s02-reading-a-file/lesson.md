## 12.2 Reading a File

With the arguments in hand, `minigrep` can open the file it's asked to search. This sub-chapter reads the file into a string, and handles the case where the file
can't be read.

* [Reading the file](#goto/01-reading-the-file): `fs::read_to_string`, `expect`, and what a missing file does.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
