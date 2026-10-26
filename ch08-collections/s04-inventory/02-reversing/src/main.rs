/// Reverses the elements of a vector in-place.
/// Vec::swap swaps two positions, and does the borrowing inside the standard library, so the caller doesn't need unsafe code.
fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0..n / 2 {
        v.swap(i, n - i - 1);
    }
}

fn main() {
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
    reverse(&mut v);
    println!("{v:?}");

    // Try: replace v.swap(i, n - i - 1) with std::mem::swap(&mut v[i], &mut v[n - i - 1]). What does the compiler say?
}
