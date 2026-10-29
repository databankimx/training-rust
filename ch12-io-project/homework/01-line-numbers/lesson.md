## Homework: Line Numbers

**The problem:** When a user searches a long file, it helps to know where each match is. Add a line number to each line the program prints, so that
`cargo run -- body poem.txt` prints:

```
1: I'm nobody! Who are you?
2: Are you nobody, too?
6: How dreary to be somebody!
```

Work in your own `minigrep` project, from VS Code. Then:

* Write `search_with_line_numbers(query: &str, contents: &str) -> Vec<(usize, &str)>` in `src/lib.rs`. It returns each matching line with its number. Line
  numbers start at 1.
* Make `run` print each result as `number: line`.
* Write a test, in the same file, that checks the numbers for a short text with a match on the first line, a match in the middle, and no match at the end.

Make the new function work with the case-sensitive search first. Adding case-insensitive support is an extension.

---

### Hints

* `lines` gives the lines of a text, and `enumerate` pairs each line with a counter. The counter starts at 0, so it needs adding one.
* The return type is a `Vec` of pairs, so `push((number, line))` adds one result.
* Test the number itself, not just the line. A test that checks only the text won't notice an off-by-one error.

---

### Check your work

For `poem.txt`, the lines with `body` are 1, 2, and 6. The output above is what your program should print.

---

### Extensions

* Add the line number option to the case-insensitive search as well. Can you avoid writing the same loop twice?
* Print the total number of matches to standard error after the results, so it doesn't mix with the output.

---

*This exercise is not from the book. It extends the minigrep project from Chapter 12.*
