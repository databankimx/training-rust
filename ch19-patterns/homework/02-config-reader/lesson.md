## Homework: A Config Reader

**The problem:** Read settings from text in the form `key=value`, one per line. Skip blank lines and comments, which start with `#`. Keep the `port` setting if it's a valid number, and report
any line that isn't in the right form.

Create a project in VS Code with `cargo new config_reader`. In `src/main.rs`:

* Write `fn parse_line(line: &str) -> Option<(&str, &str)>`. It returns the key and the value, trimmed of spaces, or `None` when the line has no `=`. Use `let...else` to split the line, with `split_once('=')`.
* Write `fn read_config(text: &str) -> (Option<u16>, Vec<String>)`. For each line, skip blank lines and comments first. For the rest, call `parse_line`. If it returns a key and value, and the key is `port`, parse the value as a `u16`, using
  `if let Ok(...)`. Any other key is ignored. A line that `parse_line` can't split, or a port that doesn't parse, is collected as an error, such as `bad line: <line>`.
* In `main`, read a sample config that includes a comment, a blank line, a valid port, and a broken line. Print the port and the errors.

Write tests in a `tests` module. Check a valid line, a comment, a blank line, and a port that is too large for a `u16`.

---

### Hints

* `let Some((key, value)) = line.split_once('=') else { return None; };` is the `let...else` form. The `else` block must return.
* A comment is a line whose first character after trimming is `#`. A blank line is empty after trimming.
* `value.parse::<u16>()` fails for numbers above 65,535, so the bad port falls into the `if let` branch's `else`.

---

### Check your work

With this sample text:

```
# server settings
port = 8080

name = demo
port = 99999
oops
```

your program should print:

```
port: 8080
errors: ["bad line: port = 99999", "bad line: oops"]
```

The `port = 99999` line is an error, because the value doesn't fit in a `u16`. `name` is a setting that the program ignores.

---

### Extensions

* Add a `timeout` setting, parsed as a number of seconds, and return it with the port.
* What is the difference between `let...else` and `if let` in the `parse_line` function? Why doesn't `if let` work for the split?

---

*This exercise is not from the book. It practices `let...else` and `if let` from Chapter 19.*
