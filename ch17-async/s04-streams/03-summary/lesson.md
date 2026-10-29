## Summary: Streams

A stream is an asynchronous sequence. It's the async counterpart of an iterator, and it works with `await`.

* **Creating:** any iterator can become a stream with `trpl::stream_from_iter`. *(See the stream from iterator step.)*
* **Using:** `next` comes from the `StreamExt` trait, so `use trpl::StreamExt` is needed. *(See the stream from iterator step.)*
* **Combining:** `filter`, `take`, and the other utility methods work as they do for iterators. *(See the filtering a stream step.)*
* **Order:** a stream's items keep their order, even when they arrive at different times. *(See the quiz below.)*

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
The `Stream` trait is most analogous to which non-async trait in the Rust standard library?

○ `Write`
○ `Index`
○ `Sync`
◉ `Iterator`
→ Yes. A stream is an asynchronous version of an iterator. Both produce a sequence of items, one at a time.
````

````quiz
Say you have a stream that produces the sequence `["A", "B", "C"]` and another stream that produces the sequence `["D", "E", "F"]`. You don't know when each stream will emit each item, only that
the items come out in the order given for each stream. When the two streams are merged and each item printed, which of the following outputs could be printed?

◉ `ABDEFC`
○ `ACBDEF`
○ `CFBEAD`
○ `FEDCBA`
→ Yes. Merging keeps the order within each stream, so `A` comes before `B`, and `B` before `C`, and `D` before `E`, and `E` before `F`. `ABDEFC` keeps both orders. The other options put `C`
before `B`, or reverse the order of the second stream.
````

---

*The summary is written for this course, and draws on section 17.4 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-04-streams.html).*
