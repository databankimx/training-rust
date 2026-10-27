## A Custom Cargo Command

Cargo can be extended with new subcommands, without changing Cargo itself. If a program named `cargo-something` is on your `PATH`, then `cargo something` runs it. The
custom command is listed by `cargo --list`, and it works like the built-in commands.

A custom command is just a binary crate with a name that begins with `cargo-`. This sample is one, named `cargo-hello`:

```rust,ignore
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(2).collect();
    println!("Hello from a custom Cargo command!");
    println!("Arguments: {args:?}");
}
```

When you run `cargo hello extra words`, Cargo runs the `cargo-hello` program, and passes two things in front of your words. The first is the program's own path, which
`env::args` always gives first. The second is `hello`, the name of the subcommand. `skip(2)` leaves out both, so the program sees only `extra` and `words`.

Run from the runner, with **Run**, there's no subcommand name, so the program receives no arguments:

```
Hello from a custom Cargo command!
Arguments: []
```

---

### Try it in a terminal

Installing the command makes it available as `cargo hello`. From this step's folder, run:

```console
cargo install --path .
cargo hello extra words
```

The output should be:

```
Hello from a custom Cargo command!
Arguments: ["extra", "words"]
```

If you installed an earlier version of the code, run `cargo install --path .` again, so the installed program picks up the change. Then run `cargo --list`. The `hello`
command appears with the built-in commands.

---

*For more detail, see section 14.5 "Extending Cargo with Custom Commands" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-05-extending-cargo.html).*
