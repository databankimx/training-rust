// Counts the words in a text. Words are separated by whitespace.
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// Returns the longest word, or None if the text has no words. When two words tie, the first one wins.
pub fn longest_word(text: &str) -> Option<&str> {
    let mut longest: Option<&str> = None;
    for word in text.split_whitespace() {
        match longest {
            Some(current) if current.len() >= word.len() => {}
            _ => longest = Some(word),
        }
    }
    longest
}
