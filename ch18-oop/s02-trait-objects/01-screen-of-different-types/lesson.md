## Using Trait Objects for Values of Different Types

A vector holds values of one type, which is a problem when the items should be different types that share a behavior. A GUI is a good example. A screen holds buttons, text fields, and
select boxes, and draws each one. Each kind of component draws differently, and a library can't know in advance every kind that a user will add.

Rust doesn't have inheritance, so the library uses a trait. `Draw` has one method, `draw`, and the `Screen` holds a vector of anything that implements it:

```rust,ignore
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

`Box<dyn Draw>` is a trait object. It's a box that holds some value implementing `Draw`, and the exact type is decided when the program runs. The `run` method calls `draw` on each component,
without knowing which type it has:

```rust,ignore
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

A user of the library can add a type of their own, which is the point of the design. `SelectBox` is defined in `main`, outside the library, and it's added to the same vector as a `Button`:

```rust,ignore
let screen = Screen {
    components: vec![
        Box::new(SelectBox { /* ... */ }),
        Box::new(Button { /* ... */ }),
    ],
};
screen.run();
```

The output is:

```
Drawing a 75x10 select box with 3 options
Drawing a 50x10 button labeled 'OK'
```

The library was written before `SelectBox` existed, and `Screen` still works with it, because `SelectBox` implements `Draw`.

---

### Trait objects and generics

The same job could be done with a generic type, `Screen<T: Draw>`. A generic type can be filled in with only one concrete type at a time, so every component of that `Screen` would have to be a
`Button`, or every one a `SelectBox`. A trait object lets one `Screen` hold a mix of types. That flexibility is the reason to choose trait objects here.

The trade-off is performance. Generics are resolved when the program is compiled, which is called static dispatch. Trait objects look up the right `draw` method when the program runs, which is
dynamic dispatch. The next step covers the type annotation that trait objects often need, and the last one covers dispatch in more detail.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest adding a `String` to the vector. The compiler rejects it, because `String` doesn't implement `Draw`. The error tells you what's missing, so you can either pass a
different type, or implement `Draw` for `String`.

---

*For more detail, see section 18.2 "Using Trait Objects That Allow for Values of Different Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).*
