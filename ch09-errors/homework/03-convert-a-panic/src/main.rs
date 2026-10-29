// Parses an age. Bad input is expected from users, so it's reported as an Err, and the caller decides what to show.
fn age_from(text: &str) -> Result<u8, String> {
    let age: u8 = text
        .trim()
        .parse()
        .map_err(|e| format!("{text:?} is not a number: {e}"))?;
    if age > 120 {
        return Err(format!("age {age} is out of range"));
    }
    Ok(age)
}

fn main() {
    for input in [" 42 ", "abc", "200"] {
        match age_from(input) {
            Ok(age) => println!("age: {age}"),
            Err(error) => println!("error: {error}"),
        }
    }
}
