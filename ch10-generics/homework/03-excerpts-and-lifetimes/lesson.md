## Homework: Excerpts and Lifetimes

**The problem:** Write two things that use lifetimes.

* A struct `Excerpt<'a>` that holds a `&'a str`, and a method `first_word(&self) -> &str` that returns the first word of its text. An empty text has no
  first word, so return an empty string in that case.
* A function `pick<'a>(first: &'a str, second: &'a str, prefer_second: bool) -> &'a str` that returns one of its two inputs, depending on the flag.

In `main`, make a `String` for the book's opening, and an `Excerpt` that borrows it. Print the first word. Then make a second `String` called `title`, and
print the result of `pick` for both values of `prefer_second`, using the excerpt's first word as the second argument.

Try writing it yourself before you look at the example. Start by deciding which lifetime each reference needs, and why.

---

### Hints

* `split_whitespace` gives the words of a string. Its first item is `next()`, and `unwrap_or("")` covers the empty case.
* In `first_word`, the lifetimes can be left out. Think about which rule the compiler applies, and why it's enough here.
* In `pick`, both inputs and the output share `'a`, because the output can be either input. Explain why `'a` must be the shorter of the two lifetimes.
* `&title` borrows the `String` as a `&str`, so it can be passed where a `&str` is expected.

---

### Test your code

Your program should print these three lines:

```
first word: Call
Call
Moby
```

The `prefer_second` flag is `true` for the first call, which returns the excerpt's word, and `false` for the second, which returns the title.

---

### Extensions

* Add a method `text_len(&self) -> usize`, and explain why it doesn't need a lifetime annotation.
* Change `pick` so the two inputs have different lifetimes, `'a` and `'b`, and the output is `'a`. Does the program still compile when `second` is shorter lived?
* Try returning `first_word` from a function that creates the `String` inside it. What does the compiler say, and what fix does the lesson suggest?

---

*This exercise is not from the book. It practices lifetimes and borrowed structs from Chapter 10.*
