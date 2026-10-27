/// Returns the element at index n of the sorted slice. The caller's slice is not changed.
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    // The sort happens on a Vec of references, so the caller's slice is never mutated.
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    t.clone()
}

fn main() {
    let v = vec![5, 4, 3, 2, 1];
    println!("{}", find_nth(&v, 2));
    println!("{:?}", v);

    // Try: call find_nth(&v, 10). What happens, and why does Rust not cause undefined behavior?
}
