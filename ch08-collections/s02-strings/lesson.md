## 8.2 Storing UTF-8 Encoded Text with Strings

Strings are more involved than they look, and that's largely because of UTF-8. This sub-chapter covers how to create and grow a `String`, how
to concatenate strings, why Rust won't index a string by number, and how to iterate over the characters or the bytes of some text.

* [Creating and appending](#goto/01-creating-and-appending): making a `String`, and adding to it with `push_str` and `push`.
* [Concatenating](#goto/02-concatenating): joining strings with `+` and `format!`, and who owns what afterwards.
* [Indexing and slicing](#goto/03-indexing-and-slicing): why a string can't be indexed by number, and how to slice by byte range.
* [Iterating](#goto/04-iterating): `chars` for characters, and `bytes` for raw data.
* [Summary](#goto/05-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
