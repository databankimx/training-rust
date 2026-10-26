## Homework: Settings

**The problem:** A settings file has one `key=value` setting per line. Write a function that parses one line, and returns the key and value. A line with
no `=`, or with an empty key, is an error, and the function returns an `Err` with a message that says what was wrong.

Write `parse_setting(line: &str) -> Result<(String, String), String>`. Then, in `main`, parse each line of the list, print the valid settings, and print the
errors. Keys and values have their spaces trimmed.

Finally, the program needs a `port`, and it must be a number. A missing or invalid port is a bug in the settings, so use `expect` for that check, with
messages that explain what's required.

Try writing it yourself before you look at the example.

---

### Hints

* `split_once('=')` splits a string at the first `=`, and returns `Option<(&str, &str)>`. `ok_or_else` turns the `None` into an error message.
* Use `?` after `ok_or_else` to return the error from `parse_setting` straight away.
* Check the key after you trim it. A key of only spaces is empty.
* `match` on each result in `main`, so that both a valid setting and an error produce output.
* For the port, the `get` returns an `Option`, and `parse` returns a `Result`. Both need a message, and `expect` is the right tool once you've decided the port is required.

---

### Test your code

Your program should print these lines:

```
port -> 8080
host -> localhost
error: no '=' in "retries"
error: empty key in " = 3"
listening on port 8080
```

---

### Extensions

* Make the function reject a value that's empty, such as `host=`. Is that a parsing error, or a validation error? Would you return it the same way?
* Replace `expect` for the port with a `match`, which prints a message and exits with an error instead of panicking. What changes for the user?
* Read the lines from a file with `fs::read_to_string`. What new error can `main` now get, and how would you handle it?

---

*This exercise is not from the book. It practices `Result`, `?`, and `expect` from Chapter 9.*
