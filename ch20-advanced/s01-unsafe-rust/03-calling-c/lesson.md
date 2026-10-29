## Calling External Code with extern

Rust programs sometimes need to call code written in another language. The `extern` keyword declares functions from another language, so that Rust can call them. This is the foreign function interface,
or FFI. The usual other language is C, and the `"C"` part of the declaration says which calling convention, called an *ABI*, the function uses.

Declaring the functions is an `unsafe extern` block. Rust can't check what the other language's functions do, so every function in the block is unsafe to call, by default:

```rust,ignore
unsafe extern "C" {
    pub safe fn abs(input: i32) -> i32;
}
```

Some C functions are safe to call. `abs` from the C standard library takes any `i32`, and has no memory to get wrong. Marking it `safe` tells Rust that it can be called without an `unsafe` block.
The `safe` keyword is a promise, and it doesn't make the function safe. If a function that is marked safe has a requirement, the promise is broken, and the responsibility is still yours.

The sample calls `abs` from `main`, with no `unsafe` block:

```rust,ignore
println!("Absolute value of -3 according to C: {}", abs(-3));
```

The output is:

```
Absolute value of -3 according to C: 3
```

The reverse also works. A Rust function can be called from C, by marking it `extern "C"`, and adding `#[unsafe(no_mangle)]` so that its name isn't changed by the compiler. The attribute is unsafe, because a name
clash with another library would be undefined behavior, and that's a check the programmer must make.

---

### Try it in the runner

Press **Run**. The output should be `Absolute value of -3 according to C: 3`.

The comments in `src/main.rs` suggest removing the word `safe` from the declaration. Now `abs` must be called in an `unsafe` block, and the compiler reports that the call requires one.

---

*For more detail, see section 20.1 "Unsafe Rust" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html).*
