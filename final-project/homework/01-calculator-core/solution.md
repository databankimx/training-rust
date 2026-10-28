## Walkthrough: Core Calculator

The example is in `src/main.rs`. Read the walkthrough after you've tried the exercise.

**Tokens.** `tokenize` walks the characters. Each token is stored with the column where it starts, so an error can point at it. A run of digits and dots becomes a number, and it's parsed at the end of the run. That's what lets `1.2.3` produce a clear error, rather than a silent failure.

**The tree.** `Expr` is an enum. A binary operation holds its operator, the column of the operator, and two boxed sub-expressions. The column is kept so that a division by zero can point at the `/`.

**The parser.** `Parser` holds the tokens and a position. It has one method for each rule of the grammar. `expr` handles `+` and `-`, and calls `term`. `term` handles `*` and `/`, and calls `factor`. `factor` handles unary minus and parentheses. The order of the calls is what gives multiplication higher precedence. Operators at the same level are handled in a loop, which is what makes them left-associative.

**Errors.** `expected` builds the message for "expected X, found Y". When the input ends, the column is one past the last character.

**The evaluator.** `evaluate` works through the tree. Division checks for zero, and reports the column of the operator.

**Testing.** The tests check each example in the spec, and each message. Starting with the tokenizer and parser tests makes the later problems easier to find.
