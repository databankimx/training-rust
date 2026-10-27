fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    // Four ways to write the same behavior. The last two have their types inferred from how they're first used.
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    println!("v1: {}", add_one_v1(5));
    println!("v2: {}", add_one_v2(5));
    println!("v3: {}", add_one_v3(5));
    println!("v4: {}", add_one_v4(5));

    // Try: call add_one_v3 with a String as well as with 5. What does the compiler say?
}
