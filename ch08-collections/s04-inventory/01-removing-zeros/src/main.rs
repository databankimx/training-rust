/// Removes all the zeros in-place from a vector of integers.
/// This version walks the indexes from the end, so removing an element never shifts one we haven't checked yet.
fn remove_zeros(v: &mut Vec<i32>) {
    for i in (0..v.len()).rev() {
        if v[i] == 0 {
            v.remove(i);
        }
    }
}

fn main() {
    let mut v = vec![1, 0, 2, 0, 3];
    remove_zeros(&mut v);
    println!("{v:?}");

    // Try: change the loop to walk forwards, `for i in 0..v.len()`. What happens near the end of the vector?
}
