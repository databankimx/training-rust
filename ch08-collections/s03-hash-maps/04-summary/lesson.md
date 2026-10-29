## Summary: Hash Maps

A hash map stores keys with associated values, and finds a value by its key. It owns the keys and values it stores, so owned values are moved in,
and `Copy` values are copied.

* **Creating and accessing:** `insert` adds a pair, and `get` returns an `Option` for a key. Using `copied` and `unwrap_or` gives a value, or a default
  when the key is missing. *(See the creating and accessing step.)*
* **Ownership and updating:** inserting a key that exists overwrites its value. `entry` with `or_insert` adds a value only when the key is missing, and
  returns a mutable reference to it, which is how counts are updated. *(See the ownership and updating step.)*
* **Grouping and borrowing:** a map can hold collections as values, and a reference from `get` borrows the whole map. *(See the grouping and hashing step.)*
* **Hashing:** the default SipHash resists denial-of-service attacks, at some cost in speed. *(See the grouping and hashing step.)*

Vectors, strings, and hash maps cover much of what programs need to store, access, and change data. The book's summary suggests three practice problems
that combine them: finding the median and mode of a list, converting words to Pig Latin, and building a company directory by department. We'll use
those as the Chapter 8 homework.

---

*The summary is adapted from the Summary of section 8.3 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).*
