fn add_one(x: i32) -> i32 {
    x + 1
}

// fn is a function pointer type. Any function with this signature can be passed in, and closures without captures coerce to it.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Stop is never constructed in this sample, so the allow attribute silences the warning.
#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    println!("The answer is: {}", do_twice(add_one, 5));

    // ToString::to_string takes &self, so it's passed to map over references, from iter().
    let numbers: Vec<u32> = (0..3).collect();
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("strings: {strings:?}");

    // An enum variant's constructor is also a function, so Status::Value can be passed to map.
    let statuses: Vec<Status> = (0u32..3).map(Status::Value).collect();
    println!("statuses: {statuses:?}");

    // Try: replace add_one with a closure that adds 2, like |x| x + 2. Does do_twice still accept it?
}
