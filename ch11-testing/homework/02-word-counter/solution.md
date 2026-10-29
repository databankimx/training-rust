## Solution: Word Counter

This walkthrough explains the example solution in `src/lib.rs`, its program in `src/main.rs`, and its integration tests in `tests/word_counter.rs`. Read it after
you've written your own version.

### Run the solution

Press **Run** for the program's output, and **Test** for the tests. The output is:

```
words: 9
longest: Some("quick")
```

The tests are four integration tests, in the `tests` section of the output.

### How it works

`count_words` uses `split_whitespace`, which splits on any run of whitespace, so tabs and newlines count as separators, and repeated spaces don't create empty words.
`count` then counts the pieces.

`longest_word` keeps the longest word so far in an `Option`. A new word replaces it only when it's strictly longer. The match guard uses `>=`, which keeps the
earlier word when two tie. That's why the sentence's longest word is `quick`, not `brown`, which has the same length.

The integration tests are in a separate crate. They can only call `count_words` and `longest_word`, because those are the only public functions. The
`use word_counter::...` line brings them into scope, just as code outside the library would.

### The extensions

Ignoring punctuation means trimming it from each word before it's counted. The tests for the sentence with a full stop would change, but the empty and whitespace
tests might not.

Counting a given word, ignoring case, can use `to_lowercase` on both sides of the comparison. Its test belongs in the same file, since it's part of the public API.

Returning an error for empty text makes the caller handle the case, rather than treating it as a count of zero. Whether that's better depends on whether an empty
text is a mistake or a normal case.

---

*This walkthrough accompanies the exercise in Chapter 11 of the course. It's not from the book.*
