## Solution: Excerpts and Lifetimes

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
first word: Call
Call
Moby
```

### How it works

`Excerpt<'a>` holds a reference, so it needs a lifetime, and an excerpt can't outlive the text it points into. In `main`, the `String` lives until the end, so
the excerpt is valid.

`first_word` takes `&self`, and returns a reference into the excerpt's text. The third elision rule gives the output the lifetime of `&self`, which is the
right answer: the word lives as long as the text the excerpt borrows. `split_whitespace().next()` gives the first word, and `unwrap_or("")` gives an empty
string when there is none. Both are string slices of the original text, so nothing is copied.

`pick` returns either input, so the output could be either one. Both inputs and the output therefore share `'a`. That means the result lives no longer than the
shorter of the two inputs, which is what the caller must be able to rely on.

In `main`, `&title` is a `&str` that borrows the `String`, and the excerpt's first word is also a `&str`. Both live long enough for the calls, so the
program compiles.

### The extensions

`text_len` takes `&self` and returns a `usize`. The return type has no references, so no output lifetime needs to be tied to anything. Elision isn't even
needed for it.

Giving the two inputs different lifetimes, `'a` and `'b`, while the output is `'a`, means the output can be used only as long as `first`. If `second` is
shorter lived, the function must not return it, so the signature would need to say that `second` must outlive `'a`, or the function must return `first` alone.
The original `'a` on both inputs is the simplest rule that's correct for both.

A function that creates a `String` and returns a reference into it is rejected: the `String` is dropped at the end of the function. The fix is to return an owned
`String`, or to take the text as a parameter, so the caller keeps it alive. The inventory in Chapter 6 used the same fix.

---

*This walkthrough accompanies the exercise in Chapter 10 of the course. It's not from the book.*
