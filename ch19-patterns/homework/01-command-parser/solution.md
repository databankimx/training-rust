## Solution: A Command Parser

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the five tests. The output is:

```
move to 3, 4
stay put at the origin
invalid coordinates
say: hello there
quit
unknown command: dance
```

### How it works

The line is split into words, and the slice of words is matched. Each arm is a slice pattern. `["move", x, y]` needs exactly three elements, with the first one equal to `"move"`. The
other arms do the same for `say` and `quit`.

The origin arm has a guard, `if *x == "0" && *y == "0"`. The guard is checked only after the pattern matches, so it only changes the result for a move. That arm must come before the general
move arm, because arms are checked in order, and the first match wins. The extension asks you to try the other order.

Inside the general move arm, a `match` on the two parsed numbers handles the case where one of them isn't a number. `(Ok(x), Ok(y))` binds both numbers, and `_` catches anything else.

`rest @ ..` binds the remaining words to `rest` while also matching any number of them. `join` puts them back together with spaces.

The final `_` arm catches everything else, and prints the original line, so the user can see what was typed.

### The extensions

A `["move", x]` arm with one coordinate would match a move with exactly two words. The pattern for it is `["move", x]`, and it should come after the three-word move arms, so it doesn't interfere with them.

If the origin arm came after the general move arm, `move 0 0` would match the general arm first, and print `move to 0, 0`. The origin arm would never run. This is the reason arms are ordered from the most specific to the most general.

---

*This walkthrough accompanies the exercise in Chapter 19 of the course. It's not from the book.*
