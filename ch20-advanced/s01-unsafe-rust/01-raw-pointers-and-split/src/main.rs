// Splits a slice in two, using raw pointers. Borrow checking can't see that the two halves don't overlap, so unsafe is needed.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    // The assertion guarantees mid is within the slice, so every pointer below is valid.
    assert!(mid <= len);

    // SAFETY: both slices lie within the original slice, and they don't overlap.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    // Raw pointers can be created in safe code. Dereferencing them needs an unsafe block.
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    // SAFETY: r1 and r2 both point to num, which is still alive.
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
    }
    println!("num is now: {num}");

    // The function is safe to call, because it wraps the unsafe code and makes only valid pointers.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v[..], 3);
    left[0] = 100;
    right[0] = 400;
    println!("left: {left:?}, right: {right:?}");

    // Try: call split_at_mut with 7 instead of 3. What happens, and why?
}
