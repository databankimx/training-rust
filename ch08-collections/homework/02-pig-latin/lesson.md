## Homework: Pig Latin

**The problem:** Write a program that translates words into Pig Latin. The user types a line of words, and each word in the line is translated. A blank
line stops the program.

The rules are:

* If a word starts with a consonant, move that first letter to the end, and add `ay`. So `first` becomes `irst-fay`.
* If a word starts with a vowel, add `hay` to the end. So `apple` becomes `apple-hay`.

Write `to_pig_latin(word: &str) -> String`, and a `main` that reads lines until the user enters a blank line. For each line, translate every word in it.

The book's note is the important part: keep UTF-8 in mind. A letter may take more than one byte, so the first letter must be taken as a `char`, not as
a byte.

Try writing it yourself before you look at the example.

---

### Hints

* `chars` gives you the characters of a word. Its first item is the first letter.
* `split_whitespace` gives you the words of a line. `map` can apply `to_pig_latin` to each one, and `join(" ")` puts them back together.
* To read a line, use `io::stdin().read_line(&mut line)`. It returns the number of bytes read, and `0` means the input has ended.
* The line includes its newline, so `trim` it before you check whether it's blank.
* `print!` doesn't add a newline, so flush the output with `io::stdout().flush()` before you read, or the prompt may not appear.

---

### Test your code

Type these lines into the input box, pressing Enter after each one, and then press Enter on a blank line. Your program should show a transcript like this:

```
Enter words to translate. Enter a blank line to quit.
> first apple Rust
irst-fay apple-hay ust-Ray
> Georgie Porgie, pudding and pie
eorgie-Gay orgie-Pay, udding-pay and-hay ie-pay
>
```

The line you type appears after each prompt. The final prompt is followed by the blank line, and the program stops.

---

### Extensions

* Lowercase each word before you translate it, so `Rust` becomes `ust-ray`. Which behavior do you think is better, and why?
* Words with punctuation, such as `Porgie,`, keep their punctuation at the end of the translated word, so the comma stays after `orgie-Pay`. Try a word
  with punctuation in the middle, such as `don't`. Should the apostrophe move as well?
* Try a word that starts with a letter outside ASCII, such as `Élan`. Does your code treat it as a vowel? Should it?

---

*This exercise is adapted from the summary of Chapter 8 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).*
