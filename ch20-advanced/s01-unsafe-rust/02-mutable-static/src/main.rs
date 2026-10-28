static mut COUNTER: u32 = 0;

// SAFETY: Calling this from more than one thread at a time is a data race, which is undefined behavior.
// Callers must make sure only one thread uses COUNTER.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // SAFETY: this program has only one thread, so nothing else can touch COUNTER at the same time.
    unsafe {
        add_to_count(3);
        // &raw const reads the value through a raw pointer, which avoids a reference to a mutable static.
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    // Try: call add_to_count(3) a second time before the println. What value is printed?
}
