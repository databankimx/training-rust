## Idioms and the as Keyword

There's a convention for how to write `use` statements, and a keyword for renaming an item when two items share a name.

---

### The idiom

When you bring in a function, it's idiomatic to bring in its parent module, and call the function through it. The call then shows where the function
lives:

```rust,ignore
use crate::front_of_house::hosting;

hosting::add_to_waitlist();
```

When you bring in a struct, enum, or other type, it's idiomatic to bring in the full path:

```rust,ignore
use std::collections::HashMap;
```

There's no deep reason for this. It's a convention that Rust code follows, and readers expect it. The exception is when two items share a name, which
is the case the rest of this step covers.

---

### Two items with the same name

Rust doesn't allow two items with the same name in one scope. The standard library has two types called `Result`: one in `std::fmt`, and one in `std::io`.
The sample brings both into scope, and gives one a new name with `as`:

```rust,ignore
use std::fmt::Result;
use std::io::Result as IoResult;
```

`as` gives the item a new local name, or alias. The second `use` makes `IoResult` mean `std::io::Result`, so the two names don't clash. The first `use`
keeps `Result` for `std::fmt::Result`.

The sample's output is:

```
Ok(()) Ok(())
```

Each function uses the name that matches its signature.

---

### The other option

You could avoid the alias by writing the parent module at each use, as in `fmt::Result` and `io::Result`. Both approaches are correct. The `as` version
is shorter when a name is used often, and the module version is clearer when the name appears rarely. The book treats both as idiomatic.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Ok(()) Ok(())
```

The comments in `src/main.rs` suggest changing `use std::io::Result as IoResult;` to `use std::io::Result;`. The compiler reports that the name `Result`
is defined twice in the same scope, because both `use` lines now bring in a `Result`.

---

*For more detail, see section 7.4 "Bringing Paths into Scope with the use Keyword" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html).*
