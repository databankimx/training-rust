## Summary: Lifetimes

Lifetimes make sure every reference is valid for as long as it's used. The compiler works them out for most code, and asks for annotations only when the relationship
between references is unclear.

* **Dangling references:** a reference must not outlive the data it points to. The borrow checker compares scopes to check this. *(See the dangling references step.)*
* **Functions:** a lifetime parameter says how the returned reference relates to the inputs. The annotation is part of the signature, and doesn't change how long
  anything lives. *(See the longest with lifetimes step.)*
* **Structs:** a struct that holds a reference declares a lifetime, and can't outlive the data its fields point to. *(See the structs and elision step.)*
* **Elision:** three rules fill in lifetimes that follow common patterns, and the compiler asks for an annotation when they don't settle the question.
  *(See the structs and elision step.)*
* **'static and generics:** `'static` means a reference lives for the whole program, and a function can take lifetimes and type parameters together.
  *(See the static and generics step.)*

Lifetimes are the last of the three tools for generic code. With generic types, traits, and lifetimes, you can write code that works across many types, and that
never leaves a dangling reference. The next sub-chapter is an inventory of the chapter's ideas, in the form of quizzes.

---

*The summary is written for this course, and draws on the discussion of lifetimes in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).*
