fn halves(v: &mut Vec<i32>) -> (&mut [i32], &mut [i32]) {
    let mid = v.len() / 2;
    v.split_at_mut(mid)
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = halves(&mut v);
    left[0] += 10;
    right[0] *= 10;
    println!("{:?}", v);
}
