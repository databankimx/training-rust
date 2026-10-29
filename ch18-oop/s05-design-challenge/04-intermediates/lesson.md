## Design Trade-offs: Intermediates

**Context:** You are designing a serialization library that converts Rust data types into formats like JSON.

**Functionality:** A `Serialize` trait that can be implemented by serializable types, and a `to_json` function that converts serializable types into JSON.

The two proposed designs are:

```rust,ignore
// Option 1: types serialize to a `Value` enum
pub enum Value {
    String(String),
    Number(isize),
}

pub trait Serialize {
    fn serialize(&self) -> Value;
}

fn value_to_json(value: Value) -> String {
    /* .. */
}

pub fn to_json(data: impl Serialize) -> String {
    let value = data.serialize();
    value_to_json(value)
}

// Option 2: types serialize via calls to a `Serializer` interface
pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: &mut S);
}

pub trait Serializer {
    fn serialize_string(&mut self, s: &str);
    fn serialize_number(&mut self, n: isize);
}

struct JsonSerializer {
    buffer: String,
}

impl Serializer for JsonSerializer {
    /* .. */
}

pub fn to_json(data: impl Serialize) -> String {
    let mut serializer = JsonSerializer {
        buffer: String::new(),
    };
    data.serialize(&mut serializer);
    serializer.buffer
}
```

---

### Requirement 1: a new data format can be added without modifying existing `Serialize` implementations

Does the design satisfy this requirement?

````quiz
Option 1, where types serialize to a `Value` enum.

◉ Yes
→ Yes. Each new format converts a `Value` into its own output, so existing `Serialize` implementations don't change. `Value` is a boundary between them.
○ No
→ Not quite. A new format only needs a new function from `Value`, so the existing implementations stay as they are.
````

````quiz
Option 2, where types serialize through a `Serializer` interface.

◉ Yes
→ Yes. Each `Serialize` implementation is generic over any `Serializer`, so a new format is a new `Serializer` implementation.
○ No
→ Not quite. A new format implements `Serializer`, and the existing `Serialize` implementations work with it unchanged.
````

---

### Requirement 2: serialization consumes as little memory as possible

Does the design satisfy this requirement?

````quiz
Option 1, where types serialize to a `Value` enum.

○ Yes
→ Not quite. The `Value` is an intermediate copy of the data, and it takes memory before the final format is written.
◉ No
→ Yes. The intermediate `Value` holds the whole data structure in memory before it's converted, which uses more memory than needed.
````

````quiz
Option 2, where types serialize through a `Serializer` interface.

◉ Yes
→ Yes. Data is written straight into the final format, with no intermediate representation.
○ No
→ Not quite. There's no intermediate `Value`, so the design uses little extra memory.
````

---

### Requirement 3: serializing the same type into several formats adds as little to the binary size as possible

Does the design satisfy this requirement?

````quiz
Option 1, where types serialize to a `Value` enum.

◉ Yes
→ Yes. There's one `serialize` for each type, which produces a `Value`, and only one conversion from `Value` per format. The code for each type is compiled once.
○ No
→ Not quite. `serialize` is not generic, so it's compiled once for each type, however many formats there are.
````

````quiz
Option 2, where types serialize through a `Serializer` interface.

○ Yes
→ Not quite. `serialize` is generic over `S`, so the compiler makes a new copy of it for each format, which grows the binary.
◉ No
→ Yes. Each new `Serializer` produces another copy of the generic `serialize` function, so the binary grows with each format.
````

---

*The design challenge is adapted from section 18.5 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-05-design-challenge.html), and the explanations draw on the book's rationale.*
