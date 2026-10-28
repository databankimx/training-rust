## 17.4 Streams: Futures in Sequence

A stream produces a sequence of items over time, which is the async form of an iterator. This sub-section turns an iterator into a stream, uses the `StreamExt` methods, and
keeps only the items it needs.

* [Creating a stream from an iterator](#goto/01-stream-from-iterator): `stream_from_iter`, `next`, and the `StreamExt` import.
* [Filtering a stream](#goto/02-filtering-a-stream): `filter` and `take`, which stop the stream early.
* [Summary](#goto/03-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
