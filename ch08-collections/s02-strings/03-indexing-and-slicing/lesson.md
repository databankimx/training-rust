## Indexing and Slicing Strings

In many languages you can read a character from a string by its position. In Rust, that doesn't compile. The reason lies in how strings are stored,
and it's worth understanding, because it shapes how you work with text.

---

### Why indexing is refused

A `String` is stored as UTF-8 bytes, and a character can take more than one byte. The Cyrillic word `Здравствуйте` has 12 characters, but 24
bytes, because each letter takes two. So a byte index doesn't always land on a character.

Suppose `&hello[0]` were allowed. The first byte of `З` is 208, and that byte alone is not a valid character. So the result would be a number that
means nothing on its own. And for an ASCII string such as `"hi"`, `&"hi"[0]` would give the byte value 104, not the letter `h`.

Rust prefers to refuse the code, rather than return a surprising value that might go unnoticed. Indexing also can't be fast in general, because finding
the nth character means walking the string from the start. Rust expects indexing to take constant time, and it can't promise that for strings.

---

### Slicing by byte range

If you know the byte positions, you can take a slice with a range. The sample takes the first four bytes:

```rust,ignore
let hello = "Здравствуйте";
let s = &hello[0..4];
```

Each letter here is two bytes, so `s` holds the first two letters, `Зд`. The sample's output is:

```
Зд
```

A slice must start and end on a character boundary. If you slice part of a character's bytes, for example `&hello[0..1]`, the program panics at
runtime. Use slicing with care, because a wrong range can stop your program.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Зд
```

The comments in `src/main.rs` suggest changing the range to `0..1`. The program panics, because byte 1 is in the middle of the first letter. The panic
message says the index is not on a character boundary.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which statement is the best explanation for why Rust does not allow string indexing?

○ Indexing strings is unsafe because it can cause a segfault or buffer overflow
→ Not quite. Rust's memory safety doesn't depend on indexing being forbidden, and the real issue is what the result should mean.
○ Indexing strings is inefficient because strings are null-terminated, so their length cannot be efficiently computed
→ Not quite. Rust strings store their length, and the inefficiency comes from finding character positions, not from the length.
○ Indexing strings would make Rust too easy to use, and Rust developers need job security
→ Not quite. That's a joke, not a technical reason.
◉ Indexing strings is ambiguous because strings represent several granularities of sequenced data
→ Yes. A string can be read as bytes, characters, or grapheme clusters. No one of these is the obvious default, so an index has no single meaning.
````

---

*For more detail, see section 8.2 "Storing UTF-8 Encoded Text with Strings" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-02-strings.html).*
