## Summary: Strings

Strings are more complicated than they first appear. A `String` is a growable, owned, UTF-8 encoded text, and it's built on a vector of bytes,
with rules that keep those bytes valid.

* **Creating and appending:** `String::new`, `to_string`, and `String::from` make a `String`. `push_str` and `push` add to it. *(See the creating and
  appending step.)*
* **Concatenating:** `+` consumes its left-hand side, and `format!` borrows all of its arguments. *(See the concatenating step.)*
* **Indexing and slicing:** a position in a string can mean a byte, a character, or a grapheme cluster, so Rust refuses to index by number. A byte
  range can still make a slice, but it must start and end on character boundaries. *(See the indexing and slicing step.)*
* **Iterating:** `chars` gives characters, and `bytes` gives raw bytes. They're different, so you choose the one your code needs. *(See the iterating step.)*

Rust makes the correct handling of text the default. That means you think about UTF-8 at the start, rather than meeting bugs with non-ASCII characters
later. The standard library offers many methods, such as `contains` and `replace`, and the documentation is the best place to find them.

---

*The summary is adapted from the closing section of 8.2 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-02-strings.html).*
