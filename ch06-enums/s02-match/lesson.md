## 6.2 The match Control Flow Construct

`match` is Rust's most powerful way to handle the possibilities of an enum. It compares a value against patterns, binds names to the
data inside, and requires every case to be covered. This sub-chapter covers how it works, and how it interacts with `Option` and ownership.

* [Match expressions](#goto/01-match-expressions): how arms are written and tried in order.
* [Binding values](#goto/02-binding-values): pulling data out of an enum variant.
* [Matching Option](#goto/03-matching-option): handling `Some` and `None` with `match`.
* [Exhaustive matches](#goto/04-exhaustive-matches): the compiler's coverage check, and catch-all patterns.
* [Ownership in match](#goto/05-ownership-in-match): when a `match` moves data, and when it borrows.
* [Summary](#goto/06-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
