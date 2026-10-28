// Builds a HashMap from key => value pairs. The optional trailing comma and the empty call are both accepted.
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

fn main() {
    let ages = hashmap! {
        "ann" => 31,
        "ben" => 25,
    };

    // HashMap has no fixed order, so the keys are sorted before printing.
    let mut keys: Vec<&&str> = ages.keys().collect();
    keys.sort();
    for key in keys {
        println!("{}: {}", key, ages[*key]);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn three_pairs_with_a_trailing_comma() {
        let map = hashmap! { "a" => 1, "b" => 2, "c" => 3, };
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn empty_call_gives_an_empty_map() {
        let map: HashMap<&str, i32> = hashmap! {};
        assert!(map.is_empty());
    }

    #[test]
    fn lookup_returns_the_value() {
        let map = hashmap! { "ann" => 31 };
        assert_eq!(map["ann"], 31);
    }
}
