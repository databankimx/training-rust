fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    // Matching on &opt borrows the value, so the String inside is not moved.
    match &opt {
        Some(s) => println!("Some: {s}"),
        None => println!("None!"),
    }

    println!("{:?}", opt);

    // Try: change &opt to opt in the match, and change the Some arm to Some(_). Does the println! still compile?
    // Then change the arm to Some(s) with opt, and try again. Why does that one fail?
}
