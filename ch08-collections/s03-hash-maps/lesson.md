## 8.3 Storing Keys with Associated Values in Hash Maps

A hash map looks up a value by a key of any type, rather than by position. This sub-chapter covers creating and reading a map, how its keys and
values are owned, how to update an entry, and how to group values into lists.

* [Creating and accessing](#goto/01-creating-and-accessing): `insert` and `get`, and giving a default for a missing key.
* [Ownership and updating](#goto/02-ownership-and-updating): owned keys and values, overwriting, and `entry` with `or_insert`.
* [Grouping and hashing](#goto/03-grouping-and-hashing): lists as values, borrows of the map, and the default hash function.
* [Summary](#goto/04-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
