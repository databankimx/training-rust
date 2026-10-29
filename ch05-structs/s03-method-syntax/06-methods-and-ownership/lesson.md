## Methods and Ownership

The last three steps covered how methods read and change data. This step shows how the three ways of taking `self` interact
with ownership, and with the borrow rules from Chapter 4. Each method's signature decides what the caller must allow.

---

### Three ways to take self

The sample has one method of each kind:

```rust,ignore
impl Rectangle {
    fn area(&self) -> u32 { /* ... */ }            // reads the instance
    fn set_width(&mut self, width: u32) { /* ... */ } // changes the instance
    fn max(self, other: Rectangle) -> Rectangle { /* ... */ } // consumes the instance
}
```

* **`&self`** borrows the instance for reading. The caller keeps ownership, and can call it again. Most methods take this form.
* **`&mut self`** borrows the instance for writing. The caller needs a mutable binding, as with any mutable borrow.
* **`self`** takes ownership. The method uses the instance up, and the caller can't use it afterwards. This form is rare, and it's usually used
  when the method turns `self` into something else.

The sample's output shows each method in turn:

```
0
10
Rectangle { width: 5, height: 2 }
```

The area starts at `0`, because the height is 2 and the width is 0. After `set_width(5)`, the area is `10`. Then `max` returns a new rectangle with
the larger width and height of the two, which is `5` and `2`.

---

### Reading and writing

A method that takes `&self` needs the read permission on the instance. An instance created with `let rect = Rectangle { ... }` has it, so `rect.area()` works.

A method that takes `&mut self` also needs the write permission, which means the binding must be declared `mut`. If you write `rect.set_width(0)` on a binding
without `mut`, Rust reports that the variable is not mutable, and suggests adding it. The same applies to a shared reference: calling `set_width` through a
`&Rectangle` is rejected, even if the underlying rectangle is mutable, because the reference doesn't allow writes.

---

### Moving with self

A method that takes `self` moves the instance into the method, unless the type implements `Copy`. The sample's `max` call does this:

```rust,ignore
let max_rect = rect.max(other);
println!("{}", rect.area());
```

Uncommenting the second line gives the error below. The call to `max` moved `rect`, so `rect` no longer has any permissions:

```text
error[E0382]: borrow of moved value: `rect`
 --> src/main.rs
   |
   | let max_rect = rect.max(other_rect);
   |                     --------------- `rect` moved due to this method call
   | println!("{}", rect.area());
   |                ^^^^^^^^^^^ value borrowed here after move
```

The line numbers depend on your version. The important phrase is "moved due to this method call".

Calling a `self` method through a reference is also rejected. The book's example is a method `set_to_max(&mut self, other: Rectangle)` that writes
`*self = self.max(other);`. Rust rejects it with "cannot move out of `*self` which is behind a mutable reference". That's the same kind of error you met in
Chapter 4.3, when moving out of a reference. The fix there was to copy or clone the value, rather than move it.

---

### Copy and moving out

A `Rectangle` holds only numbers, so moving it out of `*self` would be safe in practice. Rust still rejects the move, because `Rectangle` isn't `Copy`, and Rust doesn't assume that it is. If you add `#[derive(Copy, Clone)]` to `Rectangle`, the move becomes a copy, and `set_to_max` compiles.

Rust doesn't derive `Copy` automatically. The author of a type has to opt in, because adding a field that isn't `Copy`, such as a `String`, would silently break
every caller that relied on copying. A `String` owns heap data, and copying it would create a double-free.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
0
10
Rectangle { width: 5, height: 2 }
```

The comments in `src/main.rs` suggest uncommenting the last `println!`. Rust rejects it, because `max` took ownership of `rect`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider these two methods that increment a field of a struct. Which style would be more idiomatic for Rust?

```
struct Point(i32, i32);
impl Point {
  fn incr_v1(mut self)  { self.0 += 1; }
  fn incr_v2(&mut self) { self.0 += 1; }
}
```

○ `incr_v1`
→ Not quite. `incr_v1` takes ownership of the `Point`, so `p` can't be used after the call, which is probably not what a method that increments a field should do.
○ Both are idiomatic
→ Not quite. They both work, but one of them consumes the value without need.
○ Neither are idiomatic
→ Not quite. `incr_v2` is the usual way to write this.
◉ `incr_v2`
→ Yes. It borrows the point mutably, so the caller can keep using it after the call.
````

````quiz
Does this program compile? If it does, what does it print?

```rust
struct Point(i32, i32);
impl Point {
  fn incr_x(&mut self) {
    self.0 += 1;
  }
}

fn main() {
  let mut p = Point(0, 0);
  p.incr_x();
  println!("{}", p.0);
}
```

○ This program does not compile
→ Not quite. `p` is declared `mut`, so the mutable borrow in `incr_x` is allowed.
◉ This program does compile
→ Yes. The method changes the first field, and then `println!` reads it.
---
If it compiles, what does it print?

[ 1 ]
→ Yes. `incr_x` adds one to the first field, so `p.0` is 1 when it's printed.
````

````quiz
Does this program compile?

```rust
struct Point {
  x: i32,
  y: i32
}

impl Point {
  fn get_x(&mut self) -> &mut i32 {
    &mut self.x
  }
}

fn main() {
  let mut p = Point { x: 1, y: 2 };
  let x = p.get_x();
  *x += 1;
  println!("{} {}", *x, p.y);
}
```

○ This program does not compile
→ Yes. `get_x` borrows all of `p` mutably, so `p.y` can't be read while `x` is still in use.
○ This program does compile
→ Not quite. The method borrows the whole struct, not just `x`, so reading `p.y` while `x` is in use is rejected.
---
On which line does Rust report the problem?

[ 16 ]
→ Yes. Line 16 is the `println!`, which reads `p.y` while `x` is still in use.
````

---

*For more detail, see section 5.3 "Method Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).*
