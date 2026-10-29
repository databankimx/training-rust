## Summary: Macros

Macros are code that writes code, and they're used throughout Rust, including `println!` and `vec!`.

* **Declarative macros:** `macro_rules!` matches the source of a call against patterns, and replaces the call with the code in the matching arm. They can take a variable number of arguments. *(See the declarative macros step.)*
* **Procedural macros:** functions from tokens to tokens, in their own crate. They can analyze the code they're given, and come in three kinds: derive, attribute-like, and function-like. *(See the procedural macros step.)*
* **Choosing:** use a declarative macro for simple repetition. Use a procedural macro for derives, or when the macro must analyze the syntax of its input. *(See both steps.)*

Macros are part of everyday Rust, and most programmers will write them rarely, and read them often. Knowing the three kinds helps when an error message or a library mentions one.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

Does this program compile? If not, on which line does Rust report the problem?

```rust,ignore
macro_rules! manylet {
    ( $( $i:ident ),* = $e:expr ) => {
        $(
            let mut $i = $e;
        )*
    };
}

fn main() {
    let mut s = String::from("A");
    manylet!(x, y = s);
    x.push_str("B");
    println!("{x}{y}");
}
```

````quiz
Program result

○ This program compiles, and prints `AB`
○ This program does not compile, on line 9
○ This program does not compile, on line 10
◉ This program does not compile, on line 11
→ Yes. The expansion binds `x` and `y` to `s`, and `s` is a `String`, which isn't copied. The first binding moves `s`, so the second use on line 11 is invalid.
````

Which of these are valid reasons to write a macro as a procedural macro, rather than a declarative macro? Answer each one.

````quiz
You want to integrate with Rust's `derive` system

◉ Yes
→ Yes. Only procedural macros can define custom `derive` macros.
○ No
→ Not quite. A custom `derive` needs a procedural macro.
````

````quiz
Your macro needs nontrivial analysis of the macro user's syntax

◉ Yes
→ Yes. Procedural macros receive the code as tokens, which they can analyze in full. Declarative macros can only rearrange the input.
○ No
→ Not quite. Declarative macros can't analyze syntax this way, so a procedural macro is the right choice.
````

````quiz
You want to generate a variable-length sequence of code

○ Yes
→ Not quite. Declarative macros handle repetition with `$(...)*`, as `my_vec!` does, so this isn't a reason for a procedural macro.
◉ No
→ Yes. `$(...)*` gives variable-length repetition in a declarative macro.
````

````quiz
Your macro needs an entire item as input, not just an expression

○ Yes
→ Not quite. A declarative macro can match items, so this isn't a reason to choose a procedural macro.
◉ No
→ Yes. Declarative macros can take items as input, so this doesn't require a procedural macro.
````

Which of the following best describes the input to a procedural macro?

````quiz
Input type

○ A typed control-flow graph
○ An abstract syntax tree
◉ A sequence of tokens
○ A byte string
→ Yes. Procedural macros receive a `TokenStream`, a sequence of tokens. A crate such as `syn` can turn those tokens into a syntax tree if the macro needs one.
````

---

*The summary is written for this course, and draws on section 20.5 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-05-macros.html).*
