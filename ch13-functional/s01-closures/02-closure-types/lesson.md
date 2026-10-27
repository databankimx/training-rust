## Closure Types and Syntax

Closures look like functions, but they're written with pipes around their parameters, and their types are usually inferred rather than written out. This step
compares the forms.

```rust,ignore
fn add_one_v1(x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x| { x + 1 };
let add_one_v4 = |x| x + 1;
```

The first line is a function, and the second is a closure with every type written out. The third drops the types, and the fourth drops the braces too, because the
body is a single expression. All four do the same thing.

Closures don't need type annotations the way functions do, for a reason. A function's types are part of its interface, which other code relies on, so they're
written out. A closure is used close to where it's defined, and it isn't part of any interface. The compiler can infer its types from how it's used.

The compiler picks one concrete type for each parameter, the first time the closure is called. If `add_one_v3` is called with a `String` after being called with a
number, the types don't match, and the compiler reports an error.

The sample prints:

```
v1: 6
v2: 6
v3: 6
v4: 6
```

---

### Try it in the runner

Press **Run**. The four lines should match the output above.

The comments in `src/main.rs` suggest calling `add_one_v3` with a `String` as well. The compiler rejects it, because `add_one_v3` has already been used with a number.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following best describes the rationale for why Rust will infer the types of arguments and returns for closures, but not top-level functions?

○ Due to the halting problem, it is mathematically impossible for Rust to infer the types of top-level functions
→ Not quite. Rust could infer the types of top-level functions in principle, so the halting problem doesn't explain the rule.
○ For backwards compatibility with older versions of Rust
→ Not quite. The rule is a design choice for the language, not a compatibility measure.
○ Anything assignable to a variable can be type-inferred, and top-level functions cannot be assigned to a variable
→ Not quite. Top-level functions can be given a name and used like values, so this isn't the reason.
◉ Top-level functions can be part of a library's external interface, while closures cannot be directly exposed
→ Yes. A function's types are a promise to the users of a library, so they're written out. A closure is used only in a narrow context, so its types can be inferred.
````

````quiz
Rust permits pattern matching within closure arguments, including the use of the underscore. For example, you could write the following:

```rust
let f = |_| (); // sometimes called the "toilet closure"
let s = String::from("Hello");
f(s);
```

Which of the following best describes the relationship between `f` and `s` in this program?

○ `f` has no effect on `s`
→ Not quite. The argument is passed to `f`, so `s` is moved into the call, and that matters.
○ `f` reads `s` and then throws away the result
→ Not quite. The closure doesn't read `s`, because the underscore ignores the value entirely.
○ `f` captures `s` in its environment
→ Not quite. `s` is passed as an argument, not captured. The closure doesn't refer to `s` from the surrounding code.
◉ `f` causes `s` to be immediately dropped
→ Yes. Passing `s` moves it into `f`, and the underscore pattern doesn't bind it, so it's dropped as soon as the call ends. The closure is much like `std::mem::drop`.
````

---

*For more detail, see section 13.1 "Closures: Anonymous Functions That Capture Their Environment" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html).*
