// Turns one line of text into a description, using slice patterns on the words.
fn describe(line: &str) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words.as_slice() {
        // The origin arm comes first. Arms are checked in order, so the general move arm below would otherwise match it too.
        ["move", x, y] if *x == "0" && *y == "0" => String::from("stay put at the origin"),
        ["move", x, y] => match (x.parse::<i32>(), y.parse::<i32>()) {
            (Ok(x), Ok(y)) => format!("move to {x}, {y}"),
            _ => String::from("invalid coordinates"),
        },
        // rest @ .. binds the remaining words as a slice.
        ["say", rest @ ..] => format!("say: {}", rest.join(" ")),
        ["quit"] => String::from("quit"),
        _ => format!("unknown command: {line}"),
    }
}

fn main() {
    let lines = ["move 3 4", "move 0 0", "move a b", "say hello there", "quit", "dance"];
    for line in lines {
        println!("{}", describe(line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_with_two_numbers() {
        assert_eq!(describe("move 3 4"), "move to 3, 4");
    }

    #[test]
    fn move_to_origin_stays_put() {
        assert_eq!(describe("move 0 0"), "stay put at the origin");
    }

    #[test]
    fn move_with_bad_coordinates() {
        assert_eq!(describe("move a b"), "invalid coordinates");
    }

    #[test]
    fn say_joins_the_rest_of_the_line() {
        assert_eq!(describe("say hello there"), "say: hello there");
    }

    #[test]
    fn unknown_command_is_reported() {
        assert_eq!(describe("dance"), "unknown command: dance");
    }
}
