## Functions

You've already been writing functions. Every program has one called `main`, and you've called
`println!`, which is a macro, not a function. In this step, you'll write your own functions, pass
values into them, and get values back out.

Functions are one of the main ways Rust programs are organised. A large program is made of
many small functions, each doing one job.

---

### Part 1: Run the program

Press **F5**. The Output pane should show:

```
Hello, world!
Another function.
The measurement is: 5h
The value of x is: 5
The value of y is: 6
```

The lines run in the order they appear in `main`. Each function is only called when `main` reaches
it, even though some are written further down the file.

The code in the parts below is taken from the program in `src/main.rs`. Each piece prints only as part of the whole program.

---

### Part 2: Define and call a function

A function is declared with `fn`, then its name, then parentheses, then a block of code in curly
brackets:

```rust
fn another_function() {
    println!("Another function.");
}
```

The name uses `snake_case`, which means lowercase words joined by underscores. To use the function,
call it by name, with parentheses, from anywhere that can see it:

```rust
another_function();
```

Rust doesn't mind where you define a function, as long as it's defined somewhere the caller can see.
Many programmers put `main` first and the helper functions after it, as in this file.

---

### Part 3: Parameters

A function can take values in, through **parameters**. Each parameter needs a name and a type:

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

This function has two parameters, `value` and `unit_label`. Rust needs the type of each one, every
time. There's no guessing.

When you call the function, the values you pass in are called **arguments**. In
`print_labeled_measurement(5, 'h');`, the argument `5` goes into `value`, and `'h'` goes into
`unit_label`. People often use "parameter" and "argument" to mean the same thing, but the
distinction is useful: the parameter is in the definition, and the argument is in the call.

---

### Part 4: Statements and expressions

This is the idea that makes the rest of the chapter make sense. Rust is an *expression-based*
language, and the difference between statements and expressions matters.

A **statement** performs an action and doesn't return a value. Creating a variable with `let` is a
statement:

```rust
let y = 6;
```

An **expression** evaluates to a value. `5 + 6` is an expression, and its value is `11`. Calling a
function is an expression. So is a block in curly brackets.

Expressions don't end with a semicolon. If you add a semicolon to the end of an expression, it
becomes a statement, and it no longer gives back a value. That's the rule to remember.

Because a `let` is a statement, it doesn't give back a value. So you can't write `let x = (let y = 6);`.
Rust won't compile that.

---

### Part 5: Return values

A function can give a value back to the code that called it. You declare the return type with an
arrow, `->`, after the parameters:

```rust
fn five() -> i32 {
    5
}
```

The return type is `i32`. The body is just `5`, with no semicolon, because it's an expression whose
value is returned. Rust doesn't name a return value. It only needs the type.

The last expression in a function's body is its return value, which is why `plus_one` works the way it
does:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

`x + 1` is the final expression, so its value is returned to the caller. Add a semicolon to the end of
it, and the function would return nothing. Rust would then refuse to compile, because the function
promises an `i32`.

---

### Part 6: Early returns

Most functions return their last expression, but you can return early with the `return` keyword:

```rust
fn five_early() -> i32 {
    return 5;
}
```

Here, `return 5;` ends the function straight away, and the value is `5`. You'll use `return` most
often when a function needs to stop partway through, which you'll see once we've covered control flow.

---

### Try it

* Add a function `fn greet() { println!("Hello from greet!"); }`, and call it from `main`.
* Change `plus_one` so it returns `x + 2`. Then change it to `x + 2;` with a semicolon. What does Rust
  say, and why?
* Write `fn double(x: i32) -> i32 { x * 2 }`, and use it to print the double of 7.

---

### Quiz

*The first four questions below are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/). The last one is our own.*

````quiz
The keyword for declaring a new function in Rust is:

[ fn ]
→ `fn` is short for function. It's used before the function's name, as in `fn five() -> i32`.
````

````quiz
Does this program compile?

```rust
fn f(x) {
    println!("{x}");
}

fn main() {
    f(0);
}
```

○ This program does compile
→ Not quite. Every parameter needs a type, so `x` must be written as `x: i32`, or another type.
◉ This program does not compile
→ Yes. Rust needs to know the type of each parameter, and `x` has no type.
````

````quiz
In Rust, a curly-brace block such as `{ ... }` is:

○ A statement only
→ Not quite. A block gives back a value, which makes it an expression.
○ A statement and a syntactic scope
→ Not quite. A block is an expression, not a statement.
◉ An expression and a syntactic scope
→ Yes. A block evaluates to its final expression, and any variables declared inside it stay inside it.
○ An expression only
→ Not quite. Blocks are also scopes: variables declared inside them go out of use at the closing brace.
````

````quiz
Does this program compile?

```rust
fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("{}", f({
        let y = 1;
        y + 1
    }));
}
```

○ This program does not compile
→ Not quite. The block is an expression, and its value (2) is valid as an argument to `f`.
◉ This program does compile
→ Yes. A block is an expression, so it can be used anywhere a value is needed, such as an argument.
---
If it compiles, what does it print?

[ 3 ]
→ Yes. The block evaluates to 2, so `f(2)` returns 3.
````

````quiz
Does this function compile?

```rust
fn five() -> i32 {
    5;
}
```

○ It does compile
→ Not quite. The function declares that it returns an `i32`, but the body ends with a statement, which returns nothing.
◉ It does not compile
→ Yes. Removing the semicolon after `5` turns it into an expression, which is what the function returns.
````

---

*For more detail, see section 3.3 "Functions" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html).*
