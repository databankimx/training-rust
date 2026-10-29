## Solution: minigrep with Iterators

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the tests. The output is:

```
lines: 2
words: ["nobody!", "nobody,"]
first: Some("I'm nobody! Who are you?")
```

The third line, `How dreary to be somebody!`, isn't a match, because `somebody` doesn't contain `nobody`. Its letters are different, so the count is 2, not 3.

### How it works

Each function is one chain. `count_matches` takes the lines, keeps those that contain the query, and counts them. `count` is a consuming method, so it returns a number
directly, and no vector is built.

`matching_words` does the same with words. `split_whitespace` gives the words, and `filter` keeps the ones that contain the query. The words that match keep their order,
because iterators are processed in order.

`first_match` uses `find`, which is a consuming method. It stops at the first line for which the closure returns `true`, so the rest of the text isn't searched. That's the
reason to prefer `find` to `filter` followed by `next`: the later lines are never looked at.

The three functions return different kinds of result, `usize`, `Vec`, and `Option`. Each return type says what the caller can expect. A count is always a number, a list may be
empty, and a first match may not exist.

### The extensions

To ignore punctuation, strip it from each word before the comparison. `trim_matches` with a closure removes characters from both ends of a word, for example
`word.trim_matches(|c: char| !c.is_alphanumeric())`. It belongs in the `map` step between `split_whitespace` and `filter`, so the `filter` tests the cleaned word.

Lowercasing each line in `count_matches` makes the search case-insensitive, but it creates a new string for every line. That costs time and memory, and the cost grows with
the size of the text. A better design would lowercase the query once, and compare it with the lines, as the minigrep search did.

---

*This walkthrough accompanies the exercise in Chapter 13 of the course. It's not from the book.*
