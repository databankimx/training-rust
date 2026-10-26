## Shorthand and Struct Update Syntax

Creating struct instances by hand can get repetitive. Two features make it shorter: the field init shorthand, and struct
update syntax. Both are common in Rust code, so it's worth knowing them well.

---

### Field init shorthand

Suppose a function builds a `User` from an email and a username. If the parameters have the same names as the fields, you
can write each field name once:

```rust,ignore
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

`email` on its own means `email: email`. The field gets the value of the parameter with the same name. This is the *field init
shorthand*. The function behaves exactly as if you had written each pair in full.

The shorthand only works when the parameter and the field have the same name. If the names differ, write the pair out.

---

### Struct update syntax

It's often useful to make a new instance that's like an existing one, but with a few fields changed. The `..` syntax does
this. It sets the fields you name, and takes the rest from another instance:

```rust,ignore
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

The `..user1` must come last. The fields you don't name get their values from `user1`, and you can list the named fields in any
order.

The sample for this step builds a user with the shorthand, then makes a second user with the update syntax:

```
another@example.com someusername123
active: true, sign-ins: 1
```

---

### The update syntax moves data

The update syntax uses `=`, like an assignment, because it moves the data. Here, `username` is a `String`, which is not `Copy`. So
its value moves from `user1` into `user2`, and `user1` is only partly usable afterwards. You can still read `user1.active`,
because `bool` is `Copy`, but `user1.username` has been moved.

If `user2` had new values for both `email` and `username`, nothing would have been moved. Then `user1` would still be fully usable.
Only the fields that are copied, or that you didn't move, stay with `user1`.

This is the same rule you met in the moves step of Chapter 4: moving a value out of a variable leaves the variable unusable for that
value.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
another@example.com someusername123
active: true, sign-ins: 1
```

The comments in `src/main.rs` suggest printing `user1.active` and `user1.sign_in_count` after the update, and then trying
`user1.username`. The first two still work, and the third is rejected, for the reason above.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let mut a = Point { x: 1, y: 2 };
  a.x += 1;
  let b = Point { y: 1, ..a };
  a.x += 1;
  println!("{}", b.x);
}
```

○ This program does not compile
→ Not quite. `..a` is valid here, because the fields it copies are all `i32`, which are `Copy`.
◉ This program does compile
→ Yes. The update syntax copies `a.x` into `b` at that point, and the later change to `a` doesn't affect `b`.
---
If it compiles, what does it print?

[ 2 ]
→ Yes. `a.x` is 2 when `b` is created, so `b.x` is 2. The second `a.x += 1` changes only `a`.
````

---

*For more detail, see section 5.1 "Defining and Instantiating Structs" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-01-defining-structs.html).*
