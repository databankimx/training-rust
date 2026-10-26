fn main() {
    let mut v = vec![1, 2, 3];

    // Copy the number out before changing the vector. The copy doesn't point into v's buffer,
    // so the push can't leave it dangling.
    let n = v[0];
    v.push(4);
    println!("{n} {:?}", v);

    // Try: change `let n = v[0];` to `let n = &v[0];` and print n after the push. Why is the program rejected?
}
