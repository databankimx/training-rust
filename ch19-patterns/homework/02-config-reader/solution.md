## Solution: A Config Reader

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the four tests. The output is:

```
port: 8080
errors: ["bad line: port = 99999", "bad line: oops"]
```

### How it works

`parse_line` uses `let...else` to split at the `=`. The `else` block returns `None`, so the rest of the function runs only when a `=` is present. This is the pattern from Chapter 19.2: the
`else` must leave the function, which guarantees that `key` and `value` are set afterward.

`read_config` handles the lines one at a time. Blank lines and comments are skipped first, because they aren't errors. Any other line goes to `parse_line`. A line with no `=` returns `None`, and the
`let...else` in `read_config` records it as an error and moves on.

The `if let Ok(number)` picks out a valid port. A value that doesn't fit in a `u16`, such as 99999, fails to parse, so the `else` branch records an error. The earlier port is kept, so a bad
line doesn't lose a good setting. Other keys, such as `name`, are ignored.

The tests check the split, the skipped lines, a valid port, and a port that is too large.

### The extensions

A `timeout` setting would use the same shape, with `if let Ok(seconds) = value.parse::<u32>()`, and a third value in the return tuple.

`let...else` is used for the split because the failure case must leave the function, which is what its `else` block does. `if let` can't do that, because its failure branch doesn't stop the function. `if let`
is right for the port, where the failure is only one of several outcomes handled in the same loop.

---

*This walkthrough accompanies the exercise in Chapter 19 of the course. It's not from the book.*
