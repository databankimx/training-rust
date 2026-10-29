// Each search is a single chain of iterator adapters, with no loop and no mutable vector.

pub fn count_matches(query: &str, contents: &str) -> usize {
    contents.lines().filter(|line| line.contains(query)).count()
}

pub fn matching_words<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.split_whitespace().filter(|word| word.contains(query)).collect()
}

pub fn first_match<'a>(query: &str, contents: &'a str) -> Option<&'a str> {
    contents.lines().find(|line| line.contains(query))
}

const POEM: &str = "I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!";

fn main() {
    let query = "nobody";
    println!("lines: {}", count_matches(query, POEM));
    println!("words: {:?}", matching_words(query, POEM));
    println!("first: {:?}", first_match(query, POEM));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_matching_lines() {
        assert_eq!(count_matches("the", "the cat\nno match\nthe end"), 2);
    }

    #[test]
    fn counts_zero_when_nothing_matches() {
        assert_eq!(count_matches("xyz", "the cat\nno match"), 0);
    }

    #[test]
    fn finds_matching_words_in_order() {
        assert_eq!(matching_words("ab", "cab abc x"), vec!["cab", "abc"]);
    }

    #[test]
    fn first_match_returns_the_first_line() {
        assert_eq!(first_match("the", "a\nthe b\nthe c"), Some("the b"));
    }

    #[test]
    fn first_match_is_none_when_nothing_matches() {
        assert_eq!(first_match("xyz", "a\nb"), None);
    }
}
