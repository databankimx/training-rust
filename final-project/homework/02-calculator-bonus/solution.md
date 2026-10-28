## Walkthrough: Bonus Items

The example is in `src/main.rs`. It builds on the core, and adds three things.

**Exponents.** `^` is a token, and `Op::Pow` is an operator. The parser handles it in `power`, which sits between `factor` and `primary`. Its right side is a `factor`, not a `primary`, so `2^3^2` parses as `2^(3^2)`, which makes it right-associative. Because unary minus is handled in `factor` above `power`, `-2^2` gives `-(2^2)`, which is -4.

**Square roots.** A name is a run of letters and digits, which becomes an `Ident` token. In `primary`, a name must be followed by `(`, `expr`, and `)`. Only `sqrt` is known, so any other name is an error, reported at the name's column. The evaluator checks for a negative argument before taking the root.

**Brackets.** `[`, `]`, `{`, and `}` are tokens. `primary` opens a group with any opening bracket, and then checks that the closing token matches it. A mismatch is reported as the error the spec gives, with the expected closing bracket and the one found.

**Rounding.** Cube roots computed with `powf` can differ from the exact answer in the last digit. The tests compare with a tolerance of 1e-9, which is the usual approach for floating-point results.
