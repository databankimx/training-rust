## Streams: Futures in Sequence

A stream is a sequence of values that arrive over time. It's the async version of an iterator. Each call to `next` returns a future, and awaiting that future gives the next item, or
`None` once the stream has ended. The async channel from section 17.2 already works this way: its receiver produces messages over time.

The API is close to `Iterator`:

- An iterator's `next` is synchronous, and gives `Some(item)` or `None` straight away.
- A stream's `next` is asynchronous, and you await it.

Because of that similarity, any iterator can become a stream with `trpl::stream_from_iter`. The sample doubles ten numbers, and loops over the resulting stream:

```rust,ignore
let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let iter = values.iter().map(|n| n * 2);
let mut stream = trpl::stream_from_iter(iter);

while let Some(value) = stream.next().await {
    println!("The value was: {value}");
}
```

Calling `stream.next()` doesn't compile on its own. The compiler reports that no method named `next` exists, because the method comes from the trait `StreamExt`, not from `Stream`:

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter`
```

`Stream` is the low-level trait, and it combines the ideas of `Iterator` and `Future`. `StreamExt` is a higher-level trait that adds methods such as `next` and the utility methods
of `Iterator` on top of it. The `Ext` in the name is a common pattern for extending a trait. The fix is the `use` line at the top of the sample:

```rust,ignore
use trpl::StreamExt;
```

With that import, the program prints:

```
The value was: 2
The value was: 4
The value was: 6
The value was: 8
The value was: 10
The value was: 12
The value was: 14
The value was: 16
The value was: 18
The value was: 20
```

Once `StreamExt` is in scope, all of its methods are available, in the same way as the `Iterator` methods.

---

### Try it in the runner

Press **Run**. The output should match the ten lines above.

The comments in `src/main.rs` suggest changing the mapping to add 100 to each value. The values change, and the count stays the same. The stream still yields one item for each input.

---

*For more detail, see section 17.4 "Streams: Futures in Sequence" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-04-streams.html).*
