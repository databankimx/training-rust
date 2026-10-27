fn main() {
    // x is declared before r, so x lives as long as r needs it. The reference is valid.
    let x = 5;
    let r = &x;
    println!("r: {r}");

    // Try: move `let x = 5;` into an inner block, and print r after that block ends. What does the compiler say?
}
