## Bonus Items

Add these to the core calculator. Try them in order, since each one uses the one before it.

**Exponents with `^`.** `2^3` is 8. Exponents bind tighter than `*` and `/`, so `2 * 3^2` is 18. They're right-associative, so `2^3^2` is `2^(3^2)`, which is 512. A minus sign applies after the power, so `-2^2` is -4. The right side of `^` can be a negative number, so `2^-1` is 0.5.

**Square roots with `sqrt(...)`.** `sqrt(16)` is 4. A negative argument is an error: `sqrt of a negative number at column 1`. Any other name is an error: `unknown function 'foo' at column 1`. A cube root can be written as `8^(1/3)`, which is about 2. Floating-point arithmetic may give `2.0000000000000004`. Decide how you'll handle that, and explain your choice in a comment.

**Square and curly brackets.** `[ ]` and `{ }` group in the same way as `( )`. A closing bracket must match its opening one, so `(2 + 3]` is an error: `expected ')', found ']' at column 7`.

Write tests for each bonus item, including its error messages.
