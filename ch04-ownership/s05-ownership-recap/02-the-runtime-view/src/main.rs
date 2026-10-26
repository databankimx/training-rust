fn main() {
    let mut a_num = 0;
    inner(&mut a_num);
    println!("a_num is now {a_num}");

    // Try: print a_box_stack_ref inside inner. Its value is the same as a_box_heap_ref, but its type
    // is different. What is the type of each one?
}

fn inner(x: &mut i32) {
    let another_num = 1;
    let a_stack_ref = &another_num;

    let a_box = Box::new(2);
    let a_box_stack_ref = &a_box;
    let a_box_heap_ref = &*a_box;

    *x += 5;

    println!("{a_stack_ref} {a_box_heap_ref} {a_box_stack_ref}");
}
