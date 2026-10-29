fn main() {
    let v1 = vec![1, 2, 3];

    // sum is a consuming method. It takes the iterator, calls next until it's finished, and returns the total.
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("sum: {total}");

    // map is an adapter. It returns a new iterator, which does nothing until collect consumes it.
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{v2:?}");

    // Try: call sum on v1.iter() a second time, after v1_iter has been used. Why can't you use v1_iter again?
}
