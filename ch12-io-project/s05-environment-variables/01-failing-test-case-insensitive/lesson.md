## Writing a Failing Test for Case-Insensitive Search

Up to now, `search` matches only the exact case of the query. `Duct` doesn't match `duct`. Some users want to ignore case, so this section adds a second search
function, `search_case_insensitive`, and uses an environment variable to choose between them.

The first step is the test, which we write before the function. The existing test is renamed to `case_sensitive`, to make the difference clear, and its text gains a
line with a capital D, so the test also checks that the case-sensitive search still ignores it:

```rust,ignore
#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}
```

The new test uses the query `rUsT`, which has mixed case. A case-insensitive search should match both `Rust:` and `Trust me.`:

```rust,ignore
#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
}
```

`search_case_insensitive` doesn't exist yet, so the first version returns an empty list. That's enough for the test to compile, and it fails.

---

### Try it in the runner

Press **Test**. The `case_sensitive` test passes, and `case_insensitive` fails. The failure shows the expected list and an empty one.

The comments in `src/lib.rs` suggest returning only `Rust:` from the new function. The test still fails, because `Trust me.` is also expected.

---

*For more detail, see section 12.5 "Working with Environment Variables" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html).*
