## Solution: Settings

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
port -> 8080
host -> localhost
error: no '=' in "retries"
error: empty key in " = 3"
listening on port 8080
```

### How it works

`parse_setting` uses `split_once('=')`, which splits at the first `=`. When there's no `=`, `split_once` returns `None`, and `ok_or_else` turns that into an
error message. The `?` returns the error from the function straight away, which is the propagation pattern from the last sub-chapter.

The key and value are trimmed, so `port = 8080` gives `port` and `8080`. A key that's empty after trimming is rejected, because ` = 3` has no name to store.

In `main`, each line is matched. A valid setting is printed and stored. An error is printed, and the loop goes on, so one bad line doesn't stop the others.

The port is different. The program can't continue without it, so the lookup and the parse both use `expect`. The messages explain what the program needs, and
the default panic output then shows the problem. Using `expect` here is the situation the book describes: a value that must be present, where a failure is
a bug in the settings rather than something the program can recover from.

### The extensions

An empty value such as `host=` is a validation error, not a parsing error. The line is well formed, but the value isn't acceptable for this setting. Keeping
the two kinds of error separate lets the caller decide how to report each one.

Using a `match` for the port in place of `expect` lets the program print a clear message, then exit with a nonzero code. `std::process::exit(1)` does that. The
user sees a friendly message, rather than a panic with a backtrace.

Reading the file adds an I/O error. That error can be returned from `main` if `main` returns a `Result`, as Chapter 9.2 showed, so the file-reading error and
the settings errors can share one error type.

---

*This walkthrough accompanies the exercise in Chapter 9 of the course. It's not from the book.*
