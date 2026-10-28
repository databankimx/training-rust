## Ownership Inventory #4

This is a set of quizzes that tests your understanding of ownership in real-world scenarios. The scenarios are inspired by common questions about Rust. There's no code to run in this step. Read each
program, and choose the answer you think is right before you read the explanation.

The three questions below all use the same function. It adds a `Display`-able value to a vector of trait objects:

```rust,ignore
use std::fmt::Display;

fn add_displayable<T: Display>(v: &mut Vec<Box<dyn Display>>, t: T) {
    v.push(Box::new(t));
}
```

Chapter 18.2 covered the `Box<dyn Trait>` part. The question here is about lifetimes, which Chapter 10 introduced. A trait object such as `Box<dyn Display>` can hold a value that has references
in it, so the vector can't outlive the values it refers to. The compiler needs to know how long `T` lives.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
If you tried to compile the `add_displayable` function above, which of the following best describes the compiler error you would get?

○ The type `T` may be dynamically sized and cannot be cast to a trait object
○ The type `T` cannot be boxed if it contains a reference
○ The type `T` does not outlive the lifetime of the reference `&mut Vec<..>`
◉ The type `T` does not live long enough when cast to a trait object
→ Yes. When the value is boxed as `dyn Display`, the trait object must not outlive `T`, but the function says nothing about how long `T` lasts. The compiler reports the parameter type `T` as not living long enough.
````

````quiz
Assume the compiler did NOT reject the `add_displayable` function. Which of these programs would pass the compiler, and could possibly cause undefined behavior when run?

```rust
let mut v: Vec<Box<dyn Display>> = Vec::new();
{
    let n = 0;
    add_displayable(&mut v, &n);
}
println!("{}", v[0]);
```

○ A program that adds a `String` into the vector, which is moved into the function
○ A program that adds the integer `5` into the vector
○ None of these programs
◉ The program above, which adds `&n` where `n` is dropped at the end of its block
→ Yes. The vector holds a reference to `n`, and `n` is dropped before the vector is used. The `println!` then reads memory that has already been freed. The `String` and the integer don't hold references, so they can't cause this.
````

````quiz
Which of the following fixes best satisfies these three criteria: the fixed function passes the compiler, it keeps the intention of the original code, and it doesn't add unnecessary cost?

○ Change the bound to `T: Display + 'static`, so `T` lives for the whole program
○ Add `T: Clone`, and clone the value before boxing it
○ Put a lifetime on the vector reference, `v: &'a mut Vec<Box<dyn Display>>`
◉ Add a lifetime parameter `'a`, with `T: Display + 'a` and `Box<dyn Display + 'a>`
→ Yes. This ties the lifetime of `T` to the trait object, so the vector can hold references as long as they live for `'a`. The `'static` bound is less flexible, because it forbids references entirely. Cloning adds a cost, and changing the reference doesn't fix the problem.
````

---

*The quizzes are adapted from the quizzes in section 18.4 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-04-inventory.html).*
