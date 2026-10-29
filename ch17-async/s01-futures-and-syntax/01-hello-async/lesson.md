## Futures and the Async Syntax

Asynchronous programming lets a program make progress on other work while it waits. Many operations are slow because they wait for something outside the program, such as a
network response or a file. Rust's main tools are *futures*, the `async` and `await` keywords, and a *runtime* that runs the futures.

A future is a value that isn't ready yet, but will be later. The `Future` trait is the common interface for all of them. Each future holds its own progress, and what "ready"
means for it. The process of checking whether a future is ready is called *polling*.

`async` marks a function or block that can pause at points marked with `await`:

```rust,ignore
async fn greet(name: &str) -> String {
    trpl::sleep(Duration::from_millis(100)).await;
    format!("Hello, {name}, from an async function!")
}
```

Calling `greet` doesn't run its body. It returns a future, and nothing happens until that future is awaited. Futures in Rust are lazy, in the same way iterators are. The compiler
warns when a future is created and never awaited, because the work would never happen.

`await` is written after the expression it waits for, which makes chains of method calls easier to read. It's also the point where the function may pause, and give control back to
the runtime.

Rust doesn't include a runtime in the standard library, so a program needs a third-party crate. This course uses `trpl`, which provides a runtime and the common async helpers.
`trpl::block_on` runs a future to completion, and blocks the current thread until it does. That's how a non-async `main` can start async work:

```console
$ cargo run
Hello, Ada, from an async function!
```

`main` can't be `async` by itself, because the runtime has to exist before any future runs. `block_on` is the bridge.

---

### Try it in the runner

Press **Run**. The output should be `Hello, Ada, from an async function!`.

The comments in `src/main.rs` suggest adding a second call for another name. Each call gets its own future, and they run one after another here, because `block_on` is
given only one.

---

*For more detail, see section 17.1 "Futures and the Async Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html).*
