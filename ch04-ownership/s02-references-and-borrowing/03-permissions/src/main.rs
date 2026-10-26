fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // Creating num borrows v. While num is in use, v can't be changed.
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);

    // num is no longer used after this point, so v gets its permissions back and can be pushed to.
    v.push(4);
    println!("Vector is now {:?}", v);

    // Try: add `println!("Again, the third element is {}", *num);` just before v.push(4).
    // Then add the same line after it. Which of the two versions compiles, and why?
}
