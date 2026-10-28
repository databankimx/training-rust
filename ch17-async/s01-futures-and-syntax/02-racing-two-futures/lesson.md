## Racing Two Futures

The book's first async program fetches two web pages at the same time, and prints the title of whichever page finishes first. That needs a network connection, and the winner
can change from run to run, so this sample uses simulated delays instead. The structure is the same, and the result is predictable.

Here is the function that the book uses to fetch a title. It needs the network, so the code is shown here rather than run:

```rust,ignore
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
```

Each `await` is a place where the function can pause. The first waits for the response headers, and the second waits for the whole body. Both are slow for different reasons.

The sample replaces the network with a delay:

```rust,ignore
async fn page_title(url: &str, delay_ms: u64) -> (String, Option<String>) {
    trpl::sleep(Duration::from_millis(delay_ms)).await;
    (url.to_string(), Some(format!("Title of {url}")))
}
```

The two futures are then raced against each other:

```rust,ignore
let fut_1 = page_title("fast-site", 100);
let fut_2 = page_title("slow-site", 300);

let (url, title) = match trpl::select(fut_1, fut_2).await {
    trpl::Either::Left(left) => left,
    trpl::Either::Right(right) => right,
};
```

`select` polls both futures, and returns when one of them is ready. Its result is an `Either`. `Left` holds the first future's output, and `Right` holds the second's. Unlike
`Result`, `Either` has no idea of success or failure. Either future may legitimately win. The loser is dropped, so its work stops.

The output is:

```
fast-site finished first
Its title is: Title of fast-site
```

The fast site's delay is 100 ms, and the slow site's is 300 ms. The gap is large enough that the order doesn't depend on the machine.

Each future is created before `select` is called, but neither one runs until `select` polls it. Creating a future doesn't start the work, which is the laziness from the last
step.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest slowing down the fast site to 500 ms. The slow site then finishes first, and the output names it.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
If you want to wait on a future in non-async code, you should do which of the following?

○ Put an `.await` directly after the future
○ Give the future to Rust's built-in async runtime
○ Wrap the future in an `async` block and `.await` the block
◉ Give the future to a third-party async runtime
→ Yes. Rust doesn't allow `.await` outside an `async` block or function, and the standard library has no runtime. A crate such as `trpl`, which uses Tokio, provides one.
````

````quiz
The async function below desugars into which type signature?

```rust
async fn calculate(nums: &[i32]) -> i32 {
  /* ... */
}
```

○ `fn calculate<'a>(nums: &'a [i32], output: &mut impl Future<Output = i32> + 'a);`
○ `fn calculate(nums: &'_ [i32]) -> i32 as Future;`
○ `fn calculate() -> impl for<'a> Future<Input = &'a [i32], Output = i32> + 'a;`
◉ `fn calculate<'a>(nums: &'a [i32]) -> impl Future<Output = i32> + 'a;`
→ Yes. An `async fn` returns an anonymous future whose output is the declared return type. The future holds the slice, so it must not outlive the lifetime `'a`.
````

---

*For more detail, see section 17.1 "Futures and the Async Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html).*
