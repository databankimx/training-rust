## Homework: A Command Parser

**The problem:** A small program reads commands typed as text, such as `move 3 4`, `say hello there`, or `quit`. Turn each line into a description, using patterns on the words.

Create a project in VS Code with `cargo new command_parser`. In `src/main.rs`:

* Write `fn describe(line: &str) -> String`. Split the line into words with `split_whitespace`, collect them into a `Vec<&str>`, and `match` on `words.as_slice()`.
* `["move", x, y]` should describe a move. Parse `x` and `y` as numbers. If they're not numbers, say the coordinates are invalid.
* `["say", rest @ ..]` should join the rest of the words with spaces, and describe the message. Use `@` to bind the slice to `rest`.
* `["quit"]` should describe quitting.
* Add a guard so that a move to the origin, `0 0`, is described as staying put.
* Any other input should describe an unknown command.

In `main`, call `describe` on the lines `move 3 4`, `move 0 0`, `move a b`, `say hello there`, `quit`, and `dance`, and print each result.

Write tests in a `tests` module that check one line of each kind.

---

### Hints

* `words.as_slice()` lets you match on the words as a slice, with `..` for any number of elements.
* `x.parse::<i32>()` returns a `Result`. Matching it with `if let Ok(...)` in an arm is one way to handle the invalid case, and an arm can contain an `if` block.
* `rest @ ..` binds the remaining elements as a slice. `rest.join(" ")` joins them.
* Put the origin check before the general move arm, because the first matching arm wins.

---

### Check your work

Your program should print:

```
move to 3, 4
stay put at the origin
invalid coordinates
say: hello there
quit
unknown command: dance
```

---

### Extensions

* Add a `["move", x]` pattern for a move with one coordinate, and describe it as an error.
* What happens if the origin arm comes after the general move arm? Try it, and say why.

---

*This exercise is not from the book. It practices slice patterns and match guards from Chapter 19.*
