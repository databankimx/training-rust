## Design Trade-offs: Trait Trees

**Context:** You are designing a simple user interface framework that consists of a tree of widgets, such as text and buttons.

**Functionality:** The API provides a `Widget` trait that defines how a widget works. The API client implements the `Widget` trait, and calls it to render a UI.

The five proposed designs are:

```rust,ignore
// Option 1: children must be Self
pub trait Widget: Sized {
    fn render(&self) -> Vec<Self>;
}

// Option 2: children are a trait parameter
pub trait Widget<Children> {
    fn render(&self) -> Vec<Children>;
}

// Option 3: children are an associated type
pub trait Widget {
    type Children: Widget;
    fn render(&self) -> Vec<Self::Children>;
}

// Option 4: children are a reference trait object
pub trait Widget {
    fn render(&self) -> Vec<&dyn Widget>;
}

// Option 5: children are a boxed trait object
pub trait Widget {
    fn render(&self) -> Vec<Box<dyn Widget>>;
}
```

---

### Requirement 1: the client provides a single `WidgetImpl` enum for all widgets, and implements `Widget` for it

The enum is a closed set of widgets, such as `Button` and `Text`, with one `Widget` implementation for the whole enum. Does the design satisfy this requirement?

````quiz
Option 1, where children must be `Self`.

◉ Yes
→ Yes. With one enum, `render` returns other `WidgetImpl` values, so `Self` is exactly right. No dynamic dispatch is needed.
○ No
→ Not quite. The enum is a closed world, and `Vec<Self>` fits it directly.
````

````quiz
Option 2, where children are a trait parameter.

○ Yes
→ Not quite. This design adds unnecessary complexity for one enum, because the children type has to be named separately from `Self`.
◉ No
→ Yes. The extra parameter is unnecessary when the only widget is the enum.
````

````quiz
Option 3, where children are an associated type.

○ Yes
→ Not quite. An associated type works, but it's more machinery than a single enum needs.
◉ No
→ Yes. The associated type adds complexity that a closed enum doesn't need.
````

````quiz
Option 4, where children are a reference trait object.

○ Yes
→ Not quite. A trait object adds dynamic dispatch that a single enum doesn't need.
◉ No
→ Yes. Trait objects are unnecessary when there's only one implementation.
````

````quiz
Option 5, where children are a boxed trait object.

○ Yes
→ Not quite. A boxed trait object adds allocation and dynamic dispatch that a single enum doesn't need.
◉ No
→ Yes. As with Option 4, the trait object is more than this requirement needs.
````

---

### Requirement 2: the client provides a separate struct for each widget, and each widget's `render` can return widgets of any type

Now each widget is its own type, such as `Button` and `Text`, and a `render` result can hold different kinds of widget. Does the design satisfy this requirement?

````quiz
Option 1, where children must be `Self`.

○ Yes
→ Not quite. `Vec<Self>` forces every child to be the same type as its parent. A `Button` could only have `Button` children.
◉ No
→ Yes. Children must be `Self`, so mixed types in one `render` result aren't possible.
````

````quiz
Option 2, where children are a trait parameter.

○ Yes
→ Not quite. A trait parameter fixes one children type for each implementation, so a `render` can't return mixed types.
◉ No
→ Yes. The children type is chosen once, so the result is still one type.
````

````quiz
Option 3, where children are an associated type.

○ Yes
→ Not quite. The associated type is fixed by each implementation, so every child is the same type.
◉ No
→ Yes. An associated type means one children type per widget type.
````

````quiz
Option 4, where children are a reference trait object.

◉ Yes
→ Yes. `&dyn Widget` lets one vector hold different widget types, through dynamic dispatch.
○ No
→ Not quite. A trait object is what allows mixed types in one result.
````

````quiz
Option 5, where children are a boxed trait object.

◉ Yes
→ Yes. `Box<dyn Widget>` also allows mixed types in one result, and owns its children.
○ No
→ Not quite. As with Option 4, the trait object allows mixed types.
````

---

### Requirement 3: only types that implement `Widget` may be returned from `render`

Does the design satisfy this requirement?

````quiz
Option 1, where children must be `Self`.

◉ Yes
→ Yes. `Vec<Self>` can only hold values of a type that implements `Widget`.
○ No
→ Not quite. `Self` is the type implementing the trait, so the requirement is met.
````

````quiz
Option 2, where children are a trait parameter.

○ Yes
→ Not quite. `Children` is a free parameter with no bound, so `render` could return a type that doesn't implement `Widget`.
◉ No
→ Yes. Nothing requires that `Children` implements `Widget`, so the requirement isn't enforced.
````

````quiz
Option 3, where children are an associated type.

◉ Yes
→ Yes. `type Children: Widget` requires every child type to implement `Widget`.
○ No
→ Not quite. The bound on the associated type enforces the requirement.
````

````quiz
Option 4, where children are a reference trait object.

◉ Yes
→ Yes. A `&dyn Widget` is always a type that implements `Widget`.
○ No
→ Not quite. The trait object type must implement `Widget`, so the requirement is met.
````

````quiz
Option 5, where children are a boxed trait object.

◉ Yes
→ Yes. A `Box<dyn Widget>` also holds only values that implement `Widget`.
○ No
→ Not quite. As with Option 4, the trait object enforces the requirement.
````

---

*The design challenge is adapted from section 18.5 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-05-design-challenge.html), and the explanations draw on the book's rationale.*
