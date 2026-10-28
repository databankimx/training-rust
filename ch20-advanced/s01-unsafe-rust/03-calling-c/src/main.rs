// Declares functions from the C standard library. Every item in an unsafe extern block is unsafe to call, unless it's marked safe.
unsafe extern "C" {
    // abs takes any i32 and has no memory-safety requirements, so it can be marked safe.
    pub safe fn abs(input: i32) -> i32;
}

fn main() {
    // No unsafe block is needed, because abs is declared safe.
    println!("Absolute value of -3 according to C: {}", abs(-3));

    // Try: remove the word safe from the declaration. What does the compiler say now?
}
