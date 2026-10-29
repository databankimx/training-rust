## Creating and Appending to Strings

Rust has two string types. `str` is the string slice, usually seen as `&str`, and it's what string literals are. `String` is a growable, owned,
UTF-8 encoded string, provided by the standard library. Most code that says "a string" means one of these two.

A `String` is a wrapper around a `Vec<u8>` with extra rules that keep the bytes valid UTF-8. So many of the operations you learned for vectors
work on strings too.

---

### Creating a String

There are three common ways to make a `String`:

```rust,ignore
let mut s = String::new();
let greeting = "initial contents".to_string();
let other = String::from("initial contents");
```

`String::new` makes an empty string. `to_string` works on any type that implements `Display`, including string literals. `String::from` makes
a `String` from a string literal, and does the same thing as `to_string`. Which one you use is a matter of style.

---

### Appending

`push_str` adds a string slice to the end of a `String`. It takes a slice, so the caller keeps ownership of what it passes in. `push` adds a
single `char`:

```rust,ignore
let mut s = String::from("foo");
s.push_str("bar");

let mut t = String::new();
t.push('l');
```

The sample uses both, and prints the results:

```
foobar
lol
initial contents
```

Both methods need `mut`, because they change the string.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
foobar
lol
initial contents
```

The comments in `src/main.rs` suggest replacing `to_string` with `String::from`. The output doesn't change, because the two calls do the same work.

---

*For more detail, see section 8.2 "Storing UTF-8 Encoded Text with Strings" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-02-strings.html).*
