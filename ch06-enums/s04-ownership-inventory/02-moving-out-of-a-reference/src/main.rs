/// Gets the string out of an option if it exists, returning a default otherwise.
/// This version matches on the reference, so nothing is moved out of arg.
fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone(),
    }
}

fn main() {
    let opt = Some(String::from("Rust"));
    println!("{}", get_or_default(&opt));
    println!("{:?}", opt);

    let none: Option<String> = None;
    println!("[{}]", get_or_default(&none));

    // Try: replace the match with `arg.unwrap()` after an is_none check, as in the original program.
    // Which error does the compiler give, and why?
}
