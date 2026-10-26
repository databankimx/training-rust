## Solution: Longest Word

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
jumped
```

### How it works

The function walks through the words, and keeps the longest one it has seen:

```rust,ignore
let mut longest = "";
for word in sentence.split_whitespace() {
    if word.len() > longest.len() {
        longest = word;
    }
}
longest
```

`split_whitespace` returns slices of the input, so `word` is always a reference into `sentence`. `longest` starts as an empty
slice, and each time a longer word appears, it's replaced with that word's slice. No `String` is created, so nothing is copied.

The signature `fn longest_word(sentence: &str) -> &str` is what makes this safe. The returned slice points into `sentence`,
so the compiler checks that the caller keeps `sentence` alive while the result is in use. If the function returned a `String`
or a reference to a local variable, the compiler would have to reject it, for the reasons covered in the fixing ownership
errors step.

The tie in the solution goes to the earlier word, because the comparison is `>` rather than `>=`. Either choice is correct,
as long as the function does one of them consistently.

---

*This walkthrough accompanies the exercise in Chapter 4 of the course. It's not from the book.*
