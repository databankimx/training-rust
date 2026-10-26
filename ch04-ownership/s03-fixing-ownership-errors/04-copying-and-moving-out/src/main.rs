fn main() {
    // i32 implements Copy, so reading through a reference copies the number out safely.
    let numbers: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &numbers[0];
    let n: i32 = *n_ref;
    println!("{n}");

    // String does not implement Copy. Cloning copies the heap data, so both values own their own copy.
    let words: Vec<String> = vec![String::from("Hello world")];
    let word_ref: &String = &words[0];
    let mut word: String = word_ref.clone();
    word.push('!');
    println!("{word}");

    // Try: replace `word_ref.clone()` with `*word_ref`, and run the program again. Rust rejects it,
    // because moving the String out of a reference would leave two owners for the same heap data.
}
