## Homework: Optional Port

**The problem:** The function below handles an optional port with a `match`. It works, but the main path, which formats the message,
sits inside an arm, and the two arms look equally important.

```rust,ignore
struct Config {
    port: Option<u16>,
}

fn describe(config: &Config) -> String {
    match config.port {
        Some(port) => format!("listening on port {port}"),
        None => String::from("no port configured, using 8080"),
    }
}
```

Rewrite `describe` with `let...else`, so that the missing-port case returns early, and the main path reads as one straight line.

Then write a `main` that creates one configuration with port 3000, and one with no port. Print the description for each.

Try rewriting it yourself before you look at the example.

---

### Hints

* `let...else` takes a pattern on the left and an expression on the right. The `else` block must leave the function, usually with `return`.
* The pattern should be `Some(port)`, matched against `config.port`. Think about whether `config.port` should be borrowed or copied.
* `u16` is `Copy`, so a copied port is cheap to bind.
* After the `let...else`, the name `port` is available for the rest of the function.

---

### Test your code

Your program should print these two lines:

```
listening on port 3000
no port configured, using 8080
```

---

### Extensions

* Rewrite `describe` with `if let` instead. Which version makes the main path clearer, and why?
* Add a `host: Option<String>` field, and a second `let...else` that returns early when it's missing. What does the function look like now?
* Change `describe` to take `Config` by value. What does the caller lose, and when would that be acceptable?

---

*This exercise is not from the book. It practices `let...else` from Chapter 6.*
