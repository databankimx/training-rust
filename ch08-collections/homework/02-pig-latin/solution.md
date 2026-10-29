## Solution: Pig Latin

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**, then type a line of words into the input box. The transcript looks like this:

```
Enter words to translate. Enter a blank line to quit.
> first apple Rust
irst-fay apple-hay ust-Ray
> Georgie Porgie, pudding and pie
eorgie-Gay orgie-Pay, udding-pay and-hay ie-pay
>
```

### How it works

`to_pig_latin` first separates the word into its letters and its trailing punctuation. `trim_end_matches` removes characters from the end while they
aren't alphabetic, and what's left is the core word. The removed part is taken from the original with `&word[letters.len()..]`, which is safe here because
the removed characters are at the end of the word.

A word with no letters, such as `--`, has nothing to translate, so the function returns it unchanged.

The first letter is taken with `chars`, and the rest is collected into a new `String`. Taking the first letter as a `char` is what keeps the function correct
for other scripts. Slicing the first byte would split a multi-byte character, and the result would be invalid text.

The vowel check uses `contains` on a string of vowels. For a vowel, the function adds `-hay` to the letters. Otherwise it puts the rest first, then the
first letter, then `ay`. The punctuation is added at the very end, so `Porgie,` becomes `orgie-Pay,`.

The loop in `main` prints a prompt with `print!`, which doesn't add a newline. `flush` pushes the prompt to the screen before the program waits for input.
`read_line` appends the line, and returns the number of bytes read. A return of `0` means the input has ended, so the loop stops in that case too.

For a non-blank line, `split_whitespace` gives each word, `map(to_pig_latin)` translates each one, and `join(" ")` joins the results back into one line.

### The extensions

Lowercasing the word before translating makes `Rust` become `ust-ray`. Keeping the capital preserves the input, and lowercasing gives consistent output. Either
is defensible, and the choice should be a deliberate one.

Punctuation in the middle of a word, such as the apostrophe in `don't`, isn't handled by the rule above. The code treats the whole word as the core, so the
apostrophe moves with the letters. Whether it should stay in place is a design choice.

`Élan` starts with a letter that isn't ASCII, and the vowel check doesn't include it, so the function treats it as a consonant. Whether that's right depends on
the rules you want. A complete treatment would need to decide how accented vowels fit into the rules.

---

*This walkthrough accompanies the exercise in Chapter 8 of the course. It's adapted from the summary of Chapter 8 in the book.*
