## Iterating Over Strings

Iterating over text needs a decision: do you want characters, or the raw bytes? The two are different, and a string can have many more bytes
than characters. The methods make the choice explicit.

---

### Characters

The `chars` method gives each Unicode scalar value as a `char`. A character can take more than one byte, but `chars` gives it to you whole:

```rust,ignore
for c in "Зд".chars() {
    println!("{c}");
}
```

The output is:

```
З
д
```

Use `chars` when you care about letters, digits, and punctuation as the user sees them.

---

### Bytes

The `bytes` method gives each raw byte as a `u8`:

```rust,ignore
for b in "Зд".bytes() {
    println!("{b}");
}
```

The output is four numbers, because each letter is two bytes:

```
208
151
208
180
```

Use `bytes` when you need the raw data, such as when you're writing to a file or a network. Remember that one character can span several bytes, so
the byte count isn't the number of characters.

---

### Grapheme clusters

A reader might call a group of characters a single letter. In Hindi, for example, the word `नमस्ते` has six `char` values, but four grapheme
clusters. The standard library doesn't provide grapheme clusters, so you need a crate from crates.io for that.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
З
д
208
151
208
180
```

The comments in `src/main.rs` suggest counting the chars and the bytes in `नमस्ते`. The byte count is larger than the char count, because the Devanagari
characters take several bytes each.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which statement best describes the difference between the types of a string slice `&str` and a byte slice `&[u8]`?

○ `&str` always points to data stored in the program binary, whereas `&[u8]` can be stored anywhere in memory
→ Not quite. A `&str` can point to any valid UTF-8 text, not only to literals in the binary.
○ `&str` cannot be further sliced, while `&[u8]` can be sliced again
→ Not quite. Both types can be sliced.
○ `&str` can be constructed from a `String`, while `&[u8]` can only come from a `Vec`
→ Not quite. Both can be made from owned values, and neither is limited to one source.
◉ `&str` points to bytes that can always be interpreted as UTF-8, whereas `&[u8]` can be any byte sequence
→ Yes. A `&str` is a promise that its bytes are valid UTF-8, so you never need to check them before printing.
````

---

*For more detail, see section 8.2 "Storing UTF-8 Encoded Text with Strings" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-02-strings.html).*
