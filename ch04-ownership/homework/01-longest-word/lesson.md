## Homework: Longest Word

**The problem:** Write a function that takes a sentence and returns a slice of its longest word.

Your function should have this signature:

```rust,ignore
fn longest_word(sentence: &str) -> &str
```

Try writing it yourself before you look at the example.

---

### Hints

* `split_whitespace` gives you the words of a `&str`, one at a time.
* Keep track of the longest word you've seen so far. Start with an empty one.
* Think about the signature. Why does the returned `&str` have to be tied to the input, and what would go wrong if it were not?
* If two words have the same length, which one should win? Decide, and make your function do what you decided.

---

### Test your function

Check these by hand before you trust it:

* `"the quick brown fox jumped"` should give `jumped`.
* `"a bb ccc"` should give `ccc`.
* An empty sentence should give an empty word.

---

### Extensions

* Return the longest word as a `String` instead. What changes in the signature, and what does the caller have to do differently?
* Ignore punctuation, so that `"jumped."` is treated as `jumped`.
* Find the longest word in a `Vec<String>`, not just in a sentence.

---

*This exercise is not from the book. It practices slices and borrowing from Chapter 4.*
