fn move_player(spaces: u8) -> String {
    format!("move {spaces} spaces")
}

fn main() {
    for roll in [3u8, 7, 9] {
        // 3 and 7 are special. Every other roll is caught by the last arm, and moves the player.
        let action = match roll {
            3 => String::from("gain a fancy hat"),
            7 => String::from("lose a fancy hat"),
            other => move_player(other),
        };
        println!("roll {roll}: {action}");
    }

    // Try: replace `other => move_player(other),` with `_ => String::from("roll again"),`.
    // Does the output change for the roll of 9?
}
