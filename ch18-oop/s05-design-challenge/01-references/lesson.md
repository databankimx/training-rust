## Design Trade-offs: References

This section is about design trade-offs in Rust. To be an effective Rust engineer, you have to know how Rust works, and also decide which of its tools suit a given job. Each question below starts with
a case study and a set of designs. Then it gives a requirement, and asks whether each design meets it. Choose the answer you think is right before you read the explanation.

The book asks you to select every design that meets each requirement. Here, each design has its own yes-or-no question, so every question has one answer.

---

### The case study

**Context:** You are designing a simple asset manager for a game engine.

**Functionality:** An API client will provide paths of assets to load, and gets back access to the loaded assets.

**Assumptions:** Loads happen synchronously and instantaneously.

The four proposed designs are:

```rust,ignore
pub struct AssetManager {
    // ...
}

// Option 1: return a reference
impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> &Asset;
}

// Option 2: return a mutable reference
impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> &mut Asset;
}

// Option 3: return a reference-counted pointer
impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> Rc<Asset>;
}

// Option 4: return a copyable handle that can be turned into a reference
#[derive(Copy, Clone)]
pub struct AssetHandle(usize);

impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> AssetHandle;
    pub fn get(&self, handle: AssetHandle) -> &Asset;
}
```

---

### Requirement 1: once loaded, an asset should be permanently immutable

Does the design satisfy this requirement?

````quiz
Option 1, which returns `&Asset`.

◉ Yes
→ Yes. A shared reference doesn't allow mutation, so the asset stays immutable.
○ No
→ Not quite. A shared reference can't change the asset, so this design satisfies the requirement.
````

````quiz
Option 2, which returns `&mut Asset`.

○ Yes
→ Not quite. A mutable reference lets the caller change the asset, so the requirement isn't met.
◉ No
→ Yes. A mutable reference can change the asset, so immutability can't be guaranteed.
````

````quiz
Option 3, which returns `Rc<Asset>`.

◉ Yes
→ Yes. An `Rc` only gives shared access, and doesn't allow mutation.
○ No
→ Not quite. An `Rc<Asset>` doesn't permit mutating operations, so this design satisfies the requirement.
````

````quiz
Option 4, which returns an `AssetHandle` that `get` turns into `&Asset`.

◉ Yes
→ Yes. The handle is a number, and `get` returns a shared reference, so nothing can mutate the asset.
○ No
→ Not quite. `get` returns `&Asset`, so the asset can't be mutated through the handle.
````

---

### Requirement 2: clients keep access to assets across several short-term borrows of the manager

Clients might write a program like this, where an asset outlives the manager borrow that loaded it:

```rust,ignore
let asset = {
    let manager = get_manager();
    manager.load("some/path")
};
process_asset(asset);
```

Does the design satisfy this requirement?

````quiz
Option 1, which returns `&Asset`.

○ Yes
→ Not quite. The returned reference is tied to the `&mut self` borrow of the manager, so the asset can't outlive that borrow. The program above is rejected.
◉ No
→ Yes. The reference only lives as long as the manager borrow, so the asset can't outlive it.
````

````quiz
Option 2, which returns `&mut Asset`.

○ Yes
→ Not quite. The mutable reference is also tied to the manager borrow, so the asset can't outlive it.
◉ No
→ Yes. Like the shared reference, the mutable reference is limited to the lifetime of the manager borrow.
````

````quiz
Option 3, which returns `Rc<Asset>`.

◉ Yes
→ Yes. An `Rc` owns its own count, and isn't tied to the lifetime of the manager borrow.
○ No
→ Not quite. The `Rc` is not tied to the manager's lifetime, so the asset can outlive the borrow.
````

````quiz
Option 4, which returns an `AssetHandle`.

◉ Yes
→ Yes. The handle is a plain number, and doesn't borrow the manager, so it can be kept after the borrow ends.
○ No
→ Not quite. The handle is not tied to the manager's lifetime, so the requirement is met.
````

---

### Requirement 3: all assets should be deallocated at a single, predictable time

Does the design satisfy this requirement?

````quiz
Option 1, which returns `&Asset`.

◉ Yes
→ Yes. The manager owns the assets, so dropping it frees them all at one time.
○ No
→ Not quite. The manager keeps exclusive ownership of the assets, so they're freed together.
````

````quiz
Option 2, which returns `&mut Asset`.

◉ Yes
→ Yes. As with Option 1, the manager owns the assets, so they're freed together when it's dropped.
○ No
→ Not quite. The manager keeps exclusive ownership of the assets, so they're freed together.
````

````quiz
Option 3, which returns `Rc<Asset>`.

○ Yes
→ Not quite. A client can keep its own `Rc`, so the asset may outlive the manager. The manager has no way to know when every owner has dropped it.
◉ No
→ Yes. Each `Rc` is a shared owner, so the asset is freed only when every owner has dropped it. The time can't be predicted by the manager.
````

````quiz
Option 4, which returns an `AssetHandle`.

◉ Yes
→ Yes. The manager owns every asset, so they're all freed when it is dropped. The caveat is that `get` may need to panic, or return `Option<&Asset>`, if a client uses a handle after the asset has been freed.
○ No
→ Not quite. The manager owns every asset, so they're all freed when it is dropped.
````

---

*The design challenge is adapted from section 18.5 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-05-design-challenge.html), and the explanations draw on the book's rationale.*
