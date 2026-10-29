## Ownership in Match

A `match` on a value can move data out of it. Whether it does depends on the patterns, and the rules from Chapter 4 apply in the same way.
This step shows the difference, and the idiomatic way to look inside an enum without taking its contents.

---

### Matching a plain value moves the data

Suppose `opt` is an `Option<String>`, and you match on `opt` itself, with a pattern that binds the string:

```rust,ignore
let opt: Option<String> = Some(String::from("Hello world"));

match opt {
    Some(s) => println!("Some: {s}"),
    None => println!("None!"),
}

println!("{:?}", opt);
```

This is rejected. The pattern `Some(s)` binds `s` by moving the `String` out of `opt`. After the `match`, the data inside `opt` has been moved,
so `opt` can't be read in the `println!`. The compiler reports that `opt` is partially moved.

The same happens with any enum that holds non-copyable data, such as a `String`. The program would be unsafe if it compiled, because the
`String` would have two owners, and it would be freed twice.

---

### Matching a reference avoids the move

The sample matches on `&opt`, a reference to the option, rather than on `opt`:

```rust,ignore
match &opt {
    Some(s) => println!("Some: {s}"),
    None => println!("None!"),
}
```

Because the value being matched is a reference, the `s` in `Some(s)` is a reference too. Rust "pushes down" the reference from the outer enum into the
binding, so `s` has type `&String`. Nothing is moved, and `opt` can be used afterwards. The output is:

```
Some: Hello world
Some("Hello world")
```

Matching on a reference is the idiomatic way to look inside an enum without taking its contents. You'll see it often in Rust code.

---

### Ignoring the data

The sample also works if the binding is replaced with `_`. A pattern `Some(_)` matches a `Some` and doesn't bind the `String`, so nothing moves even
when matching on `opt` directly. Use `_` when you only need to know which variant it is.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Some: Hello world
Some("Hello world")
```

The comments in `src/main.rs` suggest two experiments. Changing to `Some(_)` on the value directly still compiles, because nothing is bound. Changing to `Some(s)`
on the value directly, as in the first example above, is rejected, because `s` moves the string out.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
#[derive(Debug)]
enum Either {
  Left(usize),
  Right(String)
}

fn main() {
  let x = Either::Right(String::from("Hello world"));
  let value = match x {
    Either::Left(n) => n,
    Either::Right(s) => s.len()
  };
  println!("{x:?} {value}");
}
```

○ This program does not compile
→ Yes. The arm `Either::Right(s)` moves the `String` out of `x`, so `x` can't be used in the `println!`.
○ This program does compile
→ Not quite. The `Right` arm moves the string out of `x`, which makes `x` unusable afterwards.
---
On which line does Rust report the problem?

[ 13 ]
→ Yes. Line 13 is the `println!` that uses `x` after its contents have been moved.
````

---

*For more detail, see section 6.2 "The match Control Flow Construct" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-02-match.html).*
