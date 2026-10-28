## Core Calculator

Build the core of the calculator. It must do all of the following.

- Read numbers with an optional decimal part, such as `3`, `2.5`, and `0.75`.
- Apply `+`, `-`, `*`, and `/` with the usual precedence. `2 + 3 * 4` is 14.
- Apply operators of equal precedence from left to right. `10 - 4 - 3` is 3.
- Group with parentheses `( )`, which can be nested.
- Treat a minus sign at the start of a number as negation, so `-3 + 5` is 2, and `2 * -3` is -6.
- Report errors with the column where the problem is, in these forms:
  - `unexpected character '$' at column 5`
  - `division by zero at column 3`
  - `expected ')', found end of input at column 7`
  - `expected end of input, found ')' at column 6`
  - `expected a number or '(', found end of input at column 4`
  - `invalid number '1.2.3' at column 1`
- Run a read-eval-print loop. Blank lines are ignored, and `quit` exits.

Split the work into three parts. A tokenizer turns the text into tokens, with the column of each one. A parser builds an expression tree, with one function for each rule of the grammar. An evaluator works out the value of the tree.

The grammar is:

```text
expr    := term (('+' | '-') term)*
term    := factor (('*' | '/') factor)*
factor  := '-' factor | number | '(' expr ')'
```

Write tests for each example above, and for each error message.
