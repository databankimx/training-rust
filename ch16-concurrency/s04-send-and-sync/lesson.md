## 16.4 Extensible Concurrency with Send and Sync

Most concurrency in Rust comes from the standard library, and the language itself adds only two traits: `Send` and `Sync`. This sub-section shows what they check, and how a type's
design decides which of them it has.

* [Send and Sync checks](#goto/01-send-and-sync-checks): which standard library types are `Send`, `Sync`, both, or neither.
* [A type that is Send but not Sync](#goto/02-a-connection-that-is-send): a database connection that can move to a thread but not be shared, with the book's quiz.
* [Summary](#goto/03-summary): the main ideas of this sub-section.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
