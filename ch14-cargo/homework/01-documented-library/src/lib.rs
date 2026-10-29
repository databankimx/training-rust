//! Simple statistics about the words in a piece of text.

/// Returns the number of words in `text`.
///
/// Words are separated by whitespace.
///
/// # Examples
///
/// ```
/// assert_eq!(word_stats::word_count("the quick brown fox"), 4);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Returns the average length, in characters, of the words in `text`.
///
/// # Examples
///
/// ```
/// let average = word_stats::average_word_length("the quick brown fox");
/// assert_eq!(average, 4.0);
/// ```
///
/// # Panics
///
/// Panics if `text` contains no words, because an average of nothing has no meaning.
///
/// ```should_panic
/// word_stats::average_word_length("   ");
/// ```
pub fn average_word_length(text: &str) -> f64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    assert!(!words.is_empty(), "text must contain at least one word");
    let total: usize = words.iter().map(|word| word.chars().count()).sum();
    total as f64 / words.len() as f64
}

/// Returns the longest word in `text`, or `None` if there are no words.
///
/// If several words share the longest length, the last of them is returned.
///
/// # Examples
///
/// ```
/// assert_eq!(word_stats::longest_word("the quick brown fox"), Some("brown"));
/// assert_eq!(word_stats::longest_word(""), None);
/// ```
pub fn longest_word(text: &str) -> Option<&str> {
    text.split_whitespace().max_by_key(|word| word.chars().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_has_no_words() {
        assert_eq!(word_count(""), 0);
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn counts_words_across_lines_and_tabs() {
        assert_eq!(word_count("one\ntwo\tthree"), 3);
    }
}
