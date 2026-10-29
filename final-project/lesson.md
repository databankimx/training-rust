## DIY Project: A Calculator

Build a calculator that reads expressions such as `2 + 3 * (4 - 1)` from the keyboard, and prints the result. It's a small program, but it uses most of what this course has covered: enums and
patterns, `Result` and clear error messages, recursion, `Box` for recursive types, traits for display, and tests.

You'll build it in three parts, which mirror how real language tools are built:

1. **Tokenizer.** Turns the text into tokens: numbers, operators, and brackets.
2. **Parser.** Turns the tokens into an expression tree, following the rules of precedence.
3. **Evaluator.** Works out the value of the tree.

Put a small read-eval-print loop (REPL) around these, so the program keeps asking for expressions until the user types `quit`.

---

### The core (required)

Your calculator must do all of these.

- **Numbers** with an optional decimal part, such as `3`, `2.5`, and `0.75`. Use `f64` for values.
- **Operators** `+`, `-`, `*`, and `/`, with the usual precedence: `*` and `/` bind tighter than `+` and `-`, so `2 + 3 * 4` is 14.
- **Left-to-right** order for operators of equal precedence, so `10 - 4 - 3` is 3, and `8 / 4 / 2` is 1.
- **Parentheses** `( )` for grouping, which can be nested.
- **Unary minus**, so `-3 + 5` is 2, and `2 * -3` is -6.
- **Clear errors** for bad input, which name the column where the problem is. See the error messages below.
- **A REPL.** Blank lines are ignored, `quit` exits, and each result is printed on its own line.
- **Tests** for the tokenizer, the parser, and the evaluator, in a `tests` module.

#### The grammar

Your parser should follow this grammar. Each line is one rule, and `*` means "zero or more":

```text
expr    := term (('+' | '-') term)*
term    := factor (('*' | '/') factor)*
factor  := '-' factor | number | '(' expr ')'
```

Each rule calls the rules below it, so precedence falls out of the structure. Writing one function per rule is the usual way to build this, and it's called recursive descent.

#### Error messages

Columns are counted from 1, starting at the first character of the input. Your messages must match these forms:

| Input | Message |
|---|---|
| `2 + $` | `unexpected character '$' at column 5` |
| `1 / 0` | `division by zero at column 3` |
| `(2 + 3` | `expected ')', found end of input at column 7` |
| `2 + 3)` | `expected end of input, found ')' at column 6` |
| `2 +` | `expected a number or '(', found end of input at column 4` |
| `1.2.3` | `invalid number '1.2.3' at column 1` |

The column for "end of input" is one past the last character.

---

### Examples to check against

Your program should give these results. Use them as tests.

| Input | Output |
|---|---|
| `2 + 3 * 4` | `14` |
| `(2 + 3) * 4` | `20` |
| `10 - 4 - 3` | `3` |
| `8 / 4 / 2` | `1` |
| `-3 + 5` | `2` |
| `2 * -3` | `-6` |
| `10 / 4` | `2.5` |
| `1.5 * 4` | `6` |

Output for a whole number is printed without a decimal point, as `f64` prints it.

---

### Bonus items

Each bonus builds on the core. Try them in order, since each one uses ideas from the one before.

**Bonus 1: exponents with `^`.** `2^3` is 8. Exponents bind tighter than `*` and `/`, so `2 * 3^2` is 18. They're right-associative, so `2^3^2` is `2^(3^2)`, which is 512, not `(2^3)^2`, which is 64. Add a `power` rule between `factor` and the number:

```text
factor  := '-' factor | power
power   := primary ('^' factor)?
primary := number | '(' expr ')'
```

Note that the right side of `^` is a `factor`. That's what makes `2^-1` valid, and what makes the exponent right-associative. Also, `-2^2` is `-(2^2)`, which is -4, because unary minus is applied after the power.

**Bonus 2: square roots with `sqrt(...)`.** Add the function name `sqrt`, which takes one argument in parentheses, so `sqrt(16)` is 4, and `sqrt(2) * sqrt(2)` is about 2. A square root of a negative number is an error, `sqrt of a negative number at column N`. Any other name is an error, `unknown function 'name' at column N`.

Tokenizing names needs a new token, and parsing needs a new rule for a name followed by a bracket. Because a root of any degree is `x^(1/n)`, cube roots are `8^(1/3)`, which gives 2. Expect rounding: `8^(1/3)` may print as `2`, or as `2.0000000000000004`, depending on how the floating-point calculation comes out. Decide how you'll handle that, and explain it in a comment.

**Bonus 3: square and curly brackets.** Allow `[ ]` and `{ }` to group, as `( )` does. A closing bracket must match its opening one, so `(2 + 3]` is an error: `expected ')', found ']' at column 7`. This is a good test of your error handling, because the matching check is in one place.

---

### Suggested steps

Build the program one piece at a time, and test each piece before moving on.

1. Write the `Token` enum, and a `tokenize` function that returns each token with its column. Test it on `2 + 3`.
2. Write an `Expr` enum, with numbers, negation, and binary operations. Write the parser, one function per grammar rule. Test that `2 + 3 * 4` gives the right tree.
3. Write `evaluate` for the tree. Check division by zero there, and report it with its column.
4. Write the REPL in `main`.
5. Add each bonus in turn, with its own tests.

---

### What to submit

- A project that builds with `cargo run`, and runs the REPL.
- A demo mode: `cargo run -- demo` runs a fixed list of expressions, prints each one with its result or error, and exits. Include the examples from the tables above, and the error cases.
- Tests that cover each example above, and each error message.
- A short comment at the top of `main.rs` that lists which bonus items you completed.

---

*This project is not from the book. It uses ideas from Chapters 1 to 21.*
