## Function Pointers

You can pass a named function to another function, as well as a closure. The type of a function pointer is written `fn`, with a lowercase f. It's different from the `Fn` trait, which closures implement.
A function with a matching signature coerces to `fn`:

```rust,ignore
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

`do_twice` calls `f` twice, and adds the results. It accepts any function that takes an `i32` and returns an `i32`:

```rust,ignore
println!("The answer is: {}", do_twice(add_one, 5));
```

With `add_one` adding 1, the calls give 6 and 6, and the output is:

```
The answer is: 12
```

Because `fn` is a type, the parameter is written with `fn` directly. A generic parameter with an `Fn` bound would accept closures as well, and function pointers also implement all three closure traits, so a function that takes
a closure can usually be given a plain function too. Passing `fn` is the choice when the caller must use a plain function, such as when a C library expects one.

**Named functions in iterator adapters.** Adapters such as `map` take closures, and they also accept named functions. The standard library's `ToString::to_string` is named by its full path, because several
types have a `to_string` method. It takes `&self`, so it works with references, such as the items from `iter()`:

```rust,ignore
let numbers: Vec<u32> = (0..3).collect();
let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
```

Passing it to a range of `u32` values by value wouldn't compile, because the function expects a reference, and the range yields numbers.

**Enum variants as functions.** A variant like `Status::Value` is also a function that builds the variant, so it can be passed to `map`:

```rust,ignore
let statuses: Vec<Status> = (0u32..3).map(Status::Value).collect();
```

The output is:

```
The answer is: 12
strings: ["0", "1", "2"]
statuses: [Value(0), Value(1), Value(2)]
```

Named functions and inline closures often compile to the same code, so use whichever reads more clearly.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest passing a closure that adds 2, such as `|x| x + 2`. It has no captured values, so it coerces to `fn`, and `do_twice` accepts it. The output is then `14`.

---

*For more detail, see section 20.4 "Advanced Functions and Closures" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html).*
