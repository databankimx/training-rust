## Homework: A Documented Library

**The problem:** Write a small library, `word_stats`, whose public functions are documented, and whose examples run as doc tests.

Create a library in VS Code with `cargo new word_stats --lib`. In `src/lib.rs`, write:

* `pub fn word_count(text: &str) -> usize`, which returns the number of words, where words are separated by whitespace.
* `pub fn average_word_length(text: &str) -> f64`, which returns the average length of the words in characters. It panics if the text has no words, because an average of
  nothing has no meaning.
* `pub fn longest_word(text: &str) -> Option<&str>`, which returns the longest word, or `None` when there are no words.

Give each function a doc comment with a summary line. Include an `# Examples` section with at least one example, and a `# Panics` section on `average_word_length`. Make
sure every example runs as a doc test, and that the doc tests pass with `cargo test`.

Add a unit test module too, so you can check that the empty text behaves as the docs say.

---

### Hints

* Use `text.split_whitespace()` to get the words. `count` gives the number of words.
* An average is a sum divided by a count. Cast the counts to `f64` before dividing, so the division keeps its fraction.
* `longest_word` can use `max_by_key` on the words, with the length as the key. Think about which word wins when two have the same length, and say so in the docs.

---

### Check your work

Run `cargo test`. You should see the unit tests and the doc tests. Every example should appear in the doc test section, and each should pass. Then change one example so it
expects a wrong value. The doc test should fail, which is the point of running examples as tests.

---

### Extensions

* Add `#![deny(missing_docs)]` at the top of `src/lib.rs`. Does the build fail, and which item does the compiler name first?
* Add a `# Errors` section to a function that returns a `Result`, and describe when it's an error.

---

*This exercise is not from the book. It practices documentation comments and doc tests from Chapter 14.*
