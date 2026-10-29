use word_counter::{count_words, longest_word};

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    println!("words: {}", count_words(text));
    println!("longest: {:?}", longest_word(text));
}
