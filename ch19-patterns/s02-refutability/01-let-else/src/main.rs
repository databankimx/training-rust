fn main() {
    let some_option_value: Option<i32> = Some(7);

    // let...else: if the pattern doesn't match, the else block must leave the function, so x is always set afterward.
    let Some(x) = some_option_value else {
        println!("no value");
        return;
    };
    println!("x is {x}");

    // Try: change some_option_value to None. Which line prints now?
}
