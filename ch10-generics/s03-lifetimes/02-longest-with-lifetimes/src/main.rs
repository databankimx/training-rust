// 'a says the returned reference lives no longer than the shorter of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result is used only inside this block, while string2 is still alive.
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // Try: move the println! below the inner block, after string2 has gone out of scope. What does the compiler say?
}
