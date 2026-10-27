## Homework: Word Counter

**The problem:** Write a library that counts words in a text, and an integration test for it.

Create a library project in VS Code with `cargo new word_counter --lib`. Then write, in `src/lib.rs`:

* `count_words(text: &str) -> usize`, which returns the number of words. Words are separated by any whitespace.
* `longest_word(text: &str) -> Option<&str>`, which returns the longest word, or `None` if there are no words. When two words are the same length, return the
  first one.

Then create a `tests` folder next to `src`, with a file named `word_counter.rs`. In that file, write integration tests that check:

* Words separated by spaces are counted.
* Words separated by tabs and newlines are counted.
* An empty text has no words, and no longest word.
* The longest word is the first one, when two words tie.

Remember that an integration test can only use the library's public functions, and it needs `use word_counter::...` at the top.

---

### Hints

* `split_whitespace` splits a string into words, and handles spaces, tabs, and newlines.
* For `longest_word`, keep the longest word so far in an `Option`, and replace it only when the next word is strictly longer.
* Check `count_words` and `longest_word` on the same texts in your integration tests, to confirm they agree.

---

### Check your work

Add a `main.rs` that prints the count and the longest word for the sentence `the quick brown fox jumps over the lazy dog`. Your output should be:

```
words: 9
longest: Some("quick")
```

Then `cargo test` should run four integration tests, and all of them should pass.

---

### Extensions

* Ignore punctuation, so that `dog.` counts as the word `dog`. Which tests change?
* Add a function that counts how many times a given word appears, ignoring case. Test it in the same tests file.
* Make `count_words` return a `Result` that is an error for a text with no words. Is that a better design than returning `0`?

---

*This exercise is not from the book. It practices integration tests and the public API from Chapter 11.*
