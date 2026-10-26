fn main() {
    // Rust tracks borrows of each field of a tuple separately, so this compiles.
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    // Rust treats all indexes of an array as one place, so a second borrow is rejected.
    // split_at_mut splits the array into two parts that can be borrowed separately.
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
    println!("{a:?}");

    // Try: change `&a_r[0]` to `&a[2]` and remove the split. Which line does Rust reject, and why?
}
