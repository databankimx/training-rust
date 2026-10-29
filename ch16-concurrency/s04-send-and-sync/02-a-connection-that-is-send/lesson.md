## A Type That Is Send but Not Sync

Some types are safe to move between threads, but not safe to use from several threads at once. A database connection is an example. It can be handed to a worker thread,
but two threads calling `query` on the same connection at the same time could interfere with each other.

This sample has that kind of type. `DbConnection` counts its queries in a `Cell`, which lets `query` change the count through a shared reference:

```rust,ignore
struct DbConnection {
    queries_run: Cell<u32>,
}
```

`Cell<u32>` is `Send`, so `DbConnection` is `Send`. `Cell` isn't `Sync`, so `DbConnection` isn't `Sync` either. The sample moves the connection into a thread, which is allowed:

```rust,ignore
let conn = DbConnection::new();
let handle = thread::spawn(move || {
    println!("{}", conn.query("SELECT 1"));
    conn.queries_run.get()
});
```

The output is:

```
result of SELECT 1
queries run on the connection: 1
```

The main thread waits for the worker with `join`, and reads the count the worker returned. Only one thread ever uses the connection, so nothing is shared.

If the program tried to share one connection between threads, for example through `Arc<DbConnection>`, the compiler would stop it. `Arc<T>` is `Send` only when `T` is both `Send` and `Sync`, so
the `Sync` requirement fails for `DbConnection`.

The design decision follows from the traits. A connection that can be moved to a worker but not shared should be `Send` and not `Sync`, and the compiler then enforces the rule.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest sharing the connection through `Arc`. The compiler rejects it, because `Arc<DbConnection>` can only be sent to another thread if `DbConnection` is `Sync`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Imagine you are designing an API for a database connection like this:

```rust
struct DbConnection { /* ... */ }
impl DbConnection {
    fn query(&self) -> DbResult {
        /* ... */
    }
}
```

Your database does not support concurrent queries from the same connection. Which of the following marker traits should `DbConnection` implement?

◉ `Send`
○ `Send` and `Sync`
○ `Sync`
○ Neither `Send` nor `Sync`
→ Yes. It's fine to move a connection to another thread, so it should be `Send`. It must not be `Sync`, because sharing it would allow concurrent queries, which the database doesn't support.
````

---

*For more detail, see section 16.4 "Extensible Concurrency with the Sync and Send Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html).*
