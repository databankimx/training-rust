## Homework: minigrep with Iterators

**The problem:** Add two searches to `minigrep`, and write each one using only iterator adapters, with no `for` loop and no mutable vector.

Work in your own `minigrep` project, from VS Code, starting from the version at the end of section 13.3. Add these functions to `src/lib.rs`:

* `count_matches(query: &str, contents: &str) -> usize`, which returns the number of lines that contain the query.
* `matching_words<'a>(query: &str, contents: &'a str) -> Vec<&'a str>`, which returns every whitespace-separated word that contains the query, in order.
* `first_match<'a>(query: &str, contents: &'a str) -> Option<&'a str>`, which returns the first matching line, or `None` if there isn't one. Use `find`, which is
  an iterator method.

Write tests for each, in a `tests` module. Include one case with no matches, to check the `None` and zero results.

---

### Hints

* `count` counts the items an iterator produces. `lines`, then `filter`, then `count`, is one chain.
* `split_whitespace` splits a text into words, in the same way as `lines` splits it into lines.
* `find` takes a closure that returns `bool`, and returns the first item for which it's `true`, wrapped in `Some`.
* Each function should be a single expression, with no `let mut` in sight.

---

### Check your work

For the poem `I'm nobody! Who are you?`, `Are you nobody, too?`, and `How dreary to be somebody!`, searching for `nobody` should give:

```
lines: 2
words: ["nobody!", "nobody,"]
first: Some("I'm nobody! Who are you?")
```

`somebody` doesn't contain `nobody`, because its letters are different, so the third line doesn't match.

---

### Extensions

* Make `matching_words` ignore punctuation, so `nobody!` counts as a match for `nobody`. Where would you strip the punctuation in the chain?
* Write `count_matches` so it works with both cases, using `to_lowercase` in the closure. What does that do to the cost of each line?

---

*This exercise is not from the book. It practices iterator adapters on the minigrep project from Chapter 13.*
