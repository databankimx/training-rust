fn main() {
    // Only the Some case needs code, so if let is shorter than a match with an empty _ arm.
    let config_max: Option<u8> = Some(3);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // With None, the block is skipped, and nothing is printed.
    let no_max: Option<u8> = None;
    if let Some(max) = no_max {
        println!("The maximum is configured to be {max}");
    }

    println!("Done");

    // Try: add `else { println!("No maximum is configured"); }` after the second if let. What prints?
}
