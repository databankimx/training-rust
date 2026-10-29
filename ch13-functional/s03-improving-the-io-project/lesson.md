## 13.3 Improving Our I/O Project

The `minigrep` project from Chapter 12 can use iterators to be shorter and clearer. This sub-section makes three changes: `Config::build` takes an iterator of arguments,
`search` uses adapters instead of a loop, and `search` returns an iterator.

* [Config with iterators](#goto/01-config-with-iterators): taking ownership of the arguments, and removing the `clone` calls.
* [Search with adapters](#goto/02-search-with-adapters): `filter` and `collect`, with no mutable vector.
* [Returning an iterator](#goto/03-returning-an-iterator): printing each match as it's found.
* [Summary](#goto/04-summary): the main ideas of this sub-section.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
