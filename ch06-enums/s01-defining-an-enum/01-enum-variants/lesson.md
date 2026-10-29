## Enum Variants

A struct groups several values that are all present at once, such as a `Rectangle`'s width and height. An enum says that a value is
*one of* a set of possibilities. It's the right tool when a value can be only one of several kinds.

Take IP addresses. Two standards are in use, version four and version six, and an address is one or the other, never both. Both are
still IP addresses, so the code should treat them as one type. An enum fits that exactly.

---

### Defining an enum

An enum is defined with the `enum` keyword, and lists its *variants*:

```rust,ignore
enum IpAddrKind {
    V4,
    V6,
}
```

Each variant is a possible value of the enum. `IpAddrKind` is now a type you can use anywhere in your code.

---

### Creating variants

Variants are namespaced under the enum's name, with `::`:

```rust,ignore
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Both values have the type `IpAddrKind`. That means one function can accept either of them, as the sample does:

```rust,ignore
fn route(ip_kind: IpAddrKind) {
    println!("routing {ip_kind:?}");
}
```

The sample calls `route` with each variant, and prints:

```
routing V4
routing V6
```

The sample derives `Debug`, as you saw in Chapter 5.2, so that `{ip_kind:?}` can print the variant's name. The book's version doesn't print anything,
but it's easier to see the result this way.

---

### Enums and the type system

A value of type `IpAddrKind` is always one of its variants, and the compiler knows exactly which ones. If you add a variant, every `match` on the enum
has to handle it, which you'll see in Chapter 6.2. That's the advantage over using plain constants or strings: the compiler can check that you've
covered every case.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
routing V4
routing V6
```

The comments in `src/main.rs` suggest adding a third variant, `V8`. The function `route` still compiles, because it takes any `IpAddrKind`. A call
with `V8` would print `routing V8`, which shows that the variant list is the only place the new kind needs to be added.

---

*For more detail, see section 6.1 "Defining an Enum" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html).*
