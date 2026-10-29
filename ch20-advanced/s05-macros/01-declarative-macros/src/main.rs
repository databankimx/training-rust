// A simplified version of vec!. Each $x:expr matches one expression, and the $(...),* repeats the body for each one.
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            // With no arguments the push never runs, so mut would be unused. The attribute stops that warning.
            #[allow(unused_mut)]
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let numbers = my_vec![1, 2, 3];
    println!("numbers: {numbers:?}");

    // The same macro works for any type, and any number of values.
    let words = my_vec!["a", "b"];
    println!("words: {words:?}");

    let empty: Vec<i32> = my_vec![];
    println!("empty: {empty:?}");

    // Try: add a call with a value that is itself an expression, such as my_vec![1 + 1, 2 * 3]. What does it print?
}
