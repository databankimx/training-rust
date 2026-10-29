## Defining Modules

A *module* groups related code together, and controls what is visible from outside. Modules let you give a name to each group of definitions,
so that a reader can find related code by its group, rather than reading every definition in the crate.

Modules are private by default. Code inside a module can't be used from outside it, unless you mark it `pub`. This is how a library hides its
implementation details and exposes only its public interface.

---

### Declaring modules

A module is declared with the `mod` keyword, a name, and curly brackets around its body. Modules can contain other modules, as well as functions,
structs, enums, and constants:

```rust,ignore
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added a party to the waitlist");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Took an order");
        }
    }
}
```

The example is modeled on a restaurant. `front_of_house` is the part customers see, and it has two child modules, `hosting` and `serving`. The
name of each module says why its functions belong together.

---

### Privacy and pub

Inside a module, everything is private from the outside. To use an item from another module, it needs `pub`. The sample marks `hosting` as `pub` so that
`main` can reach into it, and it marks `add_to_waitlist` as `pub` so that `main` can call it:

```rust,ignore
front_of_house::hosting::add_to_waitlist();
front_of_house::serving::take_order();
```

Each `pub` says what the module exposes. Anything without `pub` stays an implementation detail, which you can change later without breaking callers.

The sample's output is:

```
Added a party to the waitlist
Took an order
```

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Added a party to the waitlist
Took an order
```

The comments in `src/main.rs` suggest removing `pub` from `mod hosting`. The call to `add_to_waitlist` is then rejected, because the module itself is
private to `front_of_house`. The next sub-chapter covers the paths used in these calls.

---

*For more detail, see section 7.2 "Defining Modules to Control Scope and Privacy" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html).*
