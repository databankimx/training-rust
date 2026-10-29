// Integration tests use the library the way outside code would: only its public functions.
use word_counter::{count_words, longest_word};

#[test]
fn counts_words_separated_by_spaces() {
    assert_eq!(count_words("one two three"), 3);
}

#[test]
fn counts_words_separated_by_tabs_and_newlines() {
    assert_eq!(count_words("one\ttwo\nthree"), 3);
}

#[test]
fn empty_text_has_no_words() {
    assert_eq!(count_words(""), 0);
    assert_eq!(longest_word(""), None);
}

#[test]
fn longest_word_is_the_first_of_equal_length() {
    assert_eq!(longest_word("cat bird ant"), Some("bird"));
}
