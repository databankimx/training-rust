## Control Flow

Programs don't just run from top to bottom. They make decisions, and they repeat work. The two
tools for that are *if expressions*, which choose what to run, and *loops*, which run code again and
again. Rust has three kinds of loop, and this step covers each of them.

---

### Part 1: Run the program

Press **F5**. The Output pane should show:

```
number is divisible by 3
The value of number is: 5
The result is 20
End count = 2
3!
2!
1!
LIFTOFF!!!
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
3!
2!
1!
LIFTOFF!!!
```

The program is in sections, each one a different piece of control flow. We'll take them in order.

Each part below is taken from the program in `src/main.rs`, and prints only as part of the whole program.

---

### Part 2: if expressions

An `if` runs a block of code only when a condition is true:

```rust
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else {
    println!("number is not divisible by 4, 3");
}
```

Rust checks the conditions in order, and runs the first block whose condition is true. It doesn't
check the rest. `number` is 6, which is not divisible by 4, but it is divisible by 3, so that block
runs.

The condition must be a `bool`, which is `true` or `false`. Rust won't treat a number as true or
false, unlike some languages. You'll see that in the quiz.

Each block of code under an `if` is called an *arm*, just like the arms of a `match`, which you'll meet
in a later chapter.

---

### Part 3: if in a let

Because `if` is an expression, it can give back a value, which you can store in a variable:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

The two arms must give back the same type. Here, both are numbers. If one arm returned a number and the
other returned text, Rust would refuse to compile, because a variable can only have one type.

---

### Part 4: Loops

A loop runs the code inside it over and over. Rust has three kinds:

* `loop` runs forever, until you tell it to stop with `break`.
* `while` runs while a condition stays true.
* `for` runs once for each item in a collection or range.

Let's look at each one.

---

### Part 5: loop, and returning a value from it

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

`break` leaves the loop. It can also give back a value, written after the `break`. Here, the loop stops
when `counter` reaches 10, and `break counter * 2` gives back 20, which is stored in `result`.

A `loop` without a `break` would run forever, so you'll always need a way out.

---

### Part 6: Labelled loops

When loops are nested, `break` leaves the innermost loop by default. To leave an outer loop, give it a
label. Labels begin with a single quote:

```rust
'counting_up: loop {
    // ...
    loop {
        // ...
        break 'counting_up;
    }
}
```

The block above is a sketch, with placeholder comments, and isn't a complete program.

`break 'counting_up` leaves the outer loop, even though it's inside the inner one. In the program, the
outer loop stops once `count` reaches 2.

---

### Part 7: while

A `while` loop checks its condition before each turn, and stops when the condition is false:

```rust
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
println!("LIFTOFF!!!");
```

This is the same as writing a `loop` with an `if` and a `break`, but it's clearer, because the condition
sits in one place.

---

### Part 8: for

A `for` loop runs once for each item. Looping over an array is the most common use:

```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {element}");
}
```

Using `for` is safer than a `while` loop that counts through an index. You can't go past the end of the
array, because the loop only visits the items that exist.

A range is written with two dots, such as `1..4`, which is 1, 2, and 3. The end is not included. Call
`.rev()` to count backwards:

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
```

---

### Try it

* Change `number` to `9`, and run the program. Which block of the `if` runs?
* Change the `break counter * 2;` line to `break counter * 3;`. What does `result` become?
* In the `while` loop, change `number != 0` to `number > 0`. Does the output change?
* Write a `for` loop that prints the numbers from 1 to 5, using a range.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
fn main() {
    let x = 1;
    let y = if x { 0 } else { 1 };
    println!("{y}");
}
```

○ This program does compile
→ Not quite. An `if` condition must be a `bool`. Rust doesn't treat the number `1` as true.
◉ This program does not compile
→ Yes. The condition is an integer, but `if` needs `true` or `false`. Write `x != 0` to fix it.
````

````quiz
True or false: these two pieces of code result in the same value for `x`.

Snippet 1: `let x = if cond { 1 } else { 2 };`

Snippet 2:

```rust
let x;

if cond {
    x = 1;
} else {
    x = 2;
}
```

◉ True
→ Yes. Both snippets compile, and both give `x` the value 1 or 2, depending on `cond`.
○ False
→ Not quite. The two snippets do the same job. The first is shorter, but the result is the same.
````

````quiz
True or false: this code terminates, so it will not loop forever.

```rust
fn main() {
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }
}
```

◉ True
→ Yes. On the first turn, `x` is 1, which is not greater than 10, so the inner loop breaks, and the outer loop breaks too.
○ False
→ Not quite. `x` is 1 on the first turn, so the `else` branch runs and both loops end.
````

````quiz
Does this program compile?

```rust
fn main() {
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");
}
```

○ This program does not compile
→ Not quite. `for x in a` loops over every element, and `sum += x` is valid, so the program compiles.
◉ This program does compile
→ Yes. The loop adds up ten values of 5, so the program compiles.
---
If it compiles, what does it print?

[ 50 ]
→ Yes. The array has ten 5s, and 10 × 5 is 50.
````

---

*For more detail, see section 3.5 "Control Flow" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-05-control-flow.html).*
