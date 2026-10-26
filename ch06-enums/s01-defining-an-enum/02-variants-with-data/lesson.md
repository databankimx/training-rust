## Variants with Data

In the last step, the variants held no data, so the enum only said which kind a value was. Often you want the value and its kind
together. An enum can do that directly: each variant can hold data of its own, and the variants don't all have to hold the same
kind of data.

---

### Putting data in the variants

The sample stores an IP address in the enum itself:

```rust,ignore
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

The `#[allow(dead_code)]` line is needed here. Without it, the compiler warns that the fields are never read. `Debug` printing counts
as reading the whole value for this purpose, but not as reading each field. Chapter 6.2 shows how to read the fields with `match`.

Each variant has its own shape. A version four address holds four `u8` numbers, and a version six address holds a `String`. You create
a value by naming the variant and giving its data in parentheses:

```rust,ignore
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

The output is:

```
home: V4(127, 0, 0, 1)
loopback: V6("::1")
```

A struct can't do this. A struct has one set of fields, so a version four address and a version six address would need the same fields, which
is why the book first tried a struct with a separate `kind` field. The enum puts the kind and its data in one value.

---

### Variant names are constructors

Each variant's name also works as a function. `IpAddr::V4` is a function that takes four `u8` values, and returns an `IpAddr`. You don't have to
write this yourself. Defining the enum gives you the constructors automatically.

---

### Variants can hold different kinds of data

The variants don't have to hold the same kind of data, or the same amount. The book's `Message` enum shows this with four variants:

```rust,ignore
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

`Quit` has no data at all. `Move` has named fields, like a struct. `Write` holds one `String`, and `ChangeColor` holds three `i32` values. All four are
values of the one type, `Message`, so a function can accept any of them. The sample doesn't use this enum, but you'll meet it again in Chapter 6.2.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
home: V4(127, 0, 0, 1)
loopback: V6("::1")
```

The comments in `src/main.rs` suggest giving the version four address only three numbers. Rust rejects that, because the `V4` variant is defined
to hold four `u8` values.

---

*For more detail, see section 6.1 "Defining an Enum" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html).*
