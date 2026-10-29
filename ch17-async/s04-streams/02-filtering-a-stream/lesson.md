## Working with a Stream: filter and take

Once `StreamExt` is in scope, a stream supports the same kinds of methods as an iterator. The methods take closures, and they return new streams, so they can be chained, as
they are with iterators in Chapter 13.

This sample makes a stream of the numbers 1 to 20, keeps the even ones, and stops after three:

```rust,ignore
let stream = trpl::stream_from_iter(1..=20);
let mut evens = stream.filter(|n| n % 2 == 0).take(3);

while let Some(value) = evens.next().await {
    println!("even: {value}");
}
```

`filter` passes each item to the closure, and keeps it when the closure returns `true`. `take(3)` yields three items, and then ends the stream. The output is:

```
even: 2
even: 4
even: 6
```

The stream doesn't read ahead. Items are produced only when `next` is awaited, so after `take` has produced its third item, the remaining numbers are never looked at.

For a real stream, such as messages arriving on a channel, these methods work the same way. They don't need to know whether the items come from memory or from a network.

---

### Try it in the runner

Press **Run**. The output should be the three even lines above.

The comments in `src/main.rs` suggest changing `take(3)` to `take(5)`. The output then has five lines, and the last one is `even: 10`.

---

*For more detail, see section 17.4 "Streams: Futures in Sequence" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-04-streams.html).*
