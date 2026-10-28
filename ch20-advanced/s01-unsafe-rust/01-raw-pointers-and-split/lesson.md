## Unsafe Rust: The Superpowers

Rust's rules are checked at compile time, and they're conservative. Sometimes the compiler rejects code that is correct, because it can't prove that it's correct. Unsafe Rust lets you
take responsibility for those cases. It's the same language, with five extra operations, called *superpowers*:

1. Dereferencing a raw pointer.
2. Calling an unsafe function or method.
3. Accessing or modifying a mutable static variable.
4. Implementing an unsafe trait.
5. Accessing fields of a union.

The `unsafe` keyword doesn't turn off the borrow checker, and it doesn't make the code in the block dangerous. It means you've checked the memory safety of that code yourself. That's why the blocks should be
as small as possible, so any memory bug has few places to hide.

**Raw pointers** are written `*const T` and `*mut T`. They're like references, but they can be null, they can point to invalid memory, and they can ignore the borrowing rules. Creating one is safe.
Reading or writing through one is not:

```rust,ignore
let mut num = 5;
let r1 = &raw const num;
let r2 = &raw mut num;

unsafe {
    println!("r1 is: {}", *r1);
    *r2 = 10;
}
```

The `&raw const` and `&raw mut` operators create the pointers. The dereferences are inside `unsafe`. The output is:

```
r1 is: 5
num is now: 10
```

**Safe abstractions.** The standard library's `split_at_mut` splits a mutable slice into two halves. The borrow checker can't see that the halves don't overlap, so a plain safe version won't compile. The
book's solution uses raw pointers, inside a function that is itself safe to call:

```rust,ignore
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

The assertion is what makes the unsafe code correct. It guarantees that every pointer the function makes lies inside the slice, so callers can use it safely. A safe function with unsafe inside it is a
common pattern, and it keeps the unsafe code away from the people who call it.

The output of the sample is:

```
left: [100, 2, 3], right: [400, 5, 6]
```

---

### Try it in the runner

Press **Run**. The output should be the three lines above.

The comments in `src/main.rs` suggest calling `split_at_mut` with 7 instead of 3. The assertion fails, and the program panics before any unsafe code runs. That's the safe abstraction doing its job.

---

*For more detail, see section 20.1 "Unsafe Rust" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html).*
