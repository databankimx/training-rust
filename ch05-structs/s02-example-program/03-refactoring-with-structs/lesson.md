## Refactoring with Structs

The tuple version grouped the values, but didn't name them. A struct does both: it groups the values, and gives the group a
name and each part a name.

---

### Defining the struct

Here the rectangle is a `Rectangle` struct, with a `width` and a `height`, both `u32`:

```rust,ignore
struct Rectangle {
    width: u32,
    height: u32,
}
```

The struct names the whole thing, and the fields name its parts. Nothing in `area` now relies on a convention about which number
comes first.

---

### Borrowing the struct

The sample creates a `Rectangle`, and passes a reference to it:

```rust,ignore
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Its output is the same as before:

```
The area of the rectangle is 1500 square pixels.
```

The parameter `rectangle` is an immutable borrow of a `Rectangle`. The function only needs to read the fields, so it borrows the
struct rather than taking ownership. That leaves `main` free to keep using `rect1` after the call, which is why the call writes `&rect1`.

The function reads `width` and `height` from the borrowed struct. Reading a field through a reference doesn't move the field's value,
which is why you'll often see structs passed by reference.

The signature now says what it means: calculate the area of a `Rectangle`, using its width and height.

---

### Why the borrow matters

Suppose the function took the struct by value, as in `fn area(rectangle: Rectangle) -> u32`. The call `area(rect1)` would then move
`rect1` into the function, and `main` couldn't use it again. Here's the program that would fail:

```rust,ignore
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let a = area(rect1);
    println!("{} * {} = {}", rect1.width, rect1.height, a);
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Rust rejects the `println!` on line 14, because `rect1` was moved by the call on line 13. The borrowed version avoids the problem entirely.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The area of the rectangle is 1500 square pixels.
```

The comments in `src/main.rs` suggest removing the `&` from the call to `area`. Rust rejects that, because `area` would take ownership of `rect1`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  let a = area(rect1);
  println!("{} * {} = {}", rect1.width, rect1.height, a);
}

fn area(rectangle: Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
```

○ This program does compile
→ Not quite. `area` takes ownership of its argument, so `rect1` is moved on line 13 and can't be used on line 14.
◉ This program does not compile
→ Yes. The call `area(rect1)` moves `rect1`, so the `println!` cannot use it.
---
On which line does Rust report the problem?

[ 14 ]
→ Yes. Line 14 is the `println!` that uses `rect1` after it has been moved.
````

---

*For more detail, see section 5.2 "An Example Program Using Structs" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-02-example-structs.html).*
