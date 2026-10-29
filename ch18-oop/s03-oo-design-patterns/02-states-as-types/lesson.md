## Encoding States as Types

The first version checks the state at runtime. Every call goes through the state object, and an invalid request is simply ignored. This version moves the checks into the type system, so an invalid
request won't compile at all.

Each state becomes its own type. A draft is a `DraftPost`, a post waiting for review is a `PendingReviewPost`, and only the published `Post` has a `content` method:

```rust,ignore
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}
```

Each transition takes the old type by value, and returns the next one:

```rust,ignore
impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

Because `self` is taken by value, the draft is consumed by `request_review`, so it can't be reused. The only way to get a `Post` is to call `approve` on a `PendingReviewPost`, and the only way to get that is to
call `request_review` on a `DraftPost`. The workflow is enforced by the types, not by checks inside the methods.

`main` has to keep the result of each transition, because each call returns a new value:

```rust,ignore
let mut post = Post::new();
post.add_text("I ate a salad for lunch today");

let post = post.request_review();
let post = post.approve();

println!("published content: '{}'", post.content());
```

The output is:

```
published content: 'I ate a salad for lunch today'
```

---

### Comparing the two versions

The first version has more flexibility at runtime, and is easier to extend in one place. The second version catches mistakes when the program is compiled. A bug such as printing the text of an
unpublished post can't reach production, because the code that does it won't build. The book notes that this comes at a cost: the transitions are no longer hidden inside `Post`, so `main` has to
handle each new value.

Both designs are valid, and the choice depends on what matters most for the code. In Rust, the type system is often the better tool for rules like these.

---

### Try it in the runner

Press **Run**. The output should be the one line above.

The comments in `src/main.rs` suggest uncommenting a line that calls `content` on a new post. `Post::new` returns a `DraftPost`, which has no `content` method. The compiler reports that the method doesn't exist, and that's the protection this design provides.

---

*For more detail, see section 18.3 "Implementing an Object-Oriented Design Pattern" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html).*
