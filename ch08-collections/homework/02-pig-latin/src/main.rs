use std::io::{self, Write};

// Words that start with a vowel get "-hay" added. Other words move their first letter to the end, then add "-ay".
// Punctuation at the end of a word stays at the end of the translated word, so "Porgie," becomes "orgie-Pay,".
fn to_pig_latin(word: &str) -> String {
    // Split the word into its letters and its trailing punctuation.
    let letters = word.trim_end_matches(|c: char| !c.is_alphabetic());
    let punctuation = &word[letters.len()..];

    // A word with no letters, such as "--", has nothing to translate.
    if letters.is_empty() {
        return word.to_string();
    }

    // chars takes whole characters, so a multi-byte first letter is handled correctly.
    let mut chars = letters.chars();
    let first = chars.next().unwrap();
    let rest: String = chars.collect();

    let translated = if "aeiouAEIOU".contains(first) {
        format!("{letters}-hay")
    } else {
        format!("{rest}-{first}ay")
    };
    format!("{translated}{punctuation}")
}

fn main() {
    println!("Enter words to translate. Enter a blank line to quit.");
    loop {
        // print! has no newline, so flush to show the prompt before waiting for input.
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();

        // A blank line, or the end of input, stops the program.
        if bytes_read == 0 || trimmed.is_empty() {
            break;
        }

        // Translate each word of the line, and join the results with spaces.
        let translated: Vec<String> = trimmed.split_whitespace().map(to_pig_latin).collect();
        println!("{}", translated.join(" "));
    }
}
