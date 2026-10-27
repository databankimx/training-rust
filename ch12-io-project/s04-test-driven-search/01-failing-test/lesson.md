## Writing a Failing Test

The library now has `Config` and `run`, so its logic can be tested directly, without running the binary. This section adds the search itself using test-driven
development, which has four steps: write a test that fails, write just enough code to pass it, tidy the code while keeping the tests passing, and repeat.

The first step is the failing test. It describes what `search` should do: take a query and some text, and return only the lines of the text that contain the query.

```rust,ignore
#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}
```

The backslash after the opening quote tells Rust not to put a newline at the start of the text. Only one of the three lines contains `duct`, so only that line should be
returned.

---

### Making the test compile

`search` has to exist before the test can compile. The first version returns an empty list, which is enough to compile, and fails the test:

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let _ = (query, contents);
    Vec::new()
}
```

The lifetime `'a` says that the returned lines borrow from `contents`, not from `query`. Chapter 10 covered this kind of signature. Without the annotation, Rust
couldn't tell which input the result comes from, and would ask for it.

The test fails with a message like this:

```
assertion `left == right` failed
  left: ["safe, fast, productive."]
 right: []
```

The left value is the expected list, and the right value is what `search` returned. Seeing the failure first confirms that the test checks something real.

---

### Try it in the runner

Press **Test**. The output should show the test as FAILED, with the message above.

The comments in `src/lib.rs` suggest working out, before you write `search`, which lines contain `duct`. Only one does, and the test expects exactly that line.

---

*For more detail, see section 12.4 "Developing the Library's Functionality with Test-Driven Development" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html).*
