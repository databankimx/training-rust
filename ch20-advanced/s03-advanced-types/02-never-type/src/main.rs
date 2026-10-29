// A function that never returns, because it always panics. Its return type is !.
fn fail(message: &str) -> ! {
    panic!("fatal: {message}");
}

fn main() {
    let inputs = ["7", "x", "12"];
    let mut total = 0;
    for input in inputs {
        // continue has type !, so it can stand in for the u32 that the Ok arm produces.
        let n: u32 = match input.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        total += n;
    }
    println!("total: {total}");

    // fail has type !, so the None arm can stand in for an i32. The panic only happens if the value is None.
    let value: i32 = match Some(3) {
        Some(v) => v,
        None => fail("missing"),
    };
    println!("value: {value}");

    // Try: change Some(3) to None. What is printed, and where does the program stop?
}
