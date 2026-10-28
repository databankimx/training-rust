fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    // A match arm: the pattern Some(i) binds the value inside.
    let x: Option<i32> = Some(5);
    match x {
        None => println!("nothing"),
        Some(i) => println!("match: {}", i + 1),
    }

    // A let statement: the tuple is destructured into three variables.
    let (a, b, c) = (1, 2, 3);
    println!("let: {a} {b} {c}");

    // An if let chain: each condition can be a different kind of check.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("favorite color: {color}");
    } else if is_tuesday {
        println!("green");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("if let: purple");
        } else {
            println!("if let: orange");
        }
    } else {
        println!("blue");
    }

    // A while let loop: runs for as long as pop returns Some.
    let mut stack = vec![1, 2, 3];
    let mut popped = Vec::new();
    while let Some(top) = stack.pop() {
        popped.push(top);
    }
    println!("while let: {popped:?}");

    // A for loop: enumerate gives a tuple of index and value, and the pattern (index, value) destructures it.
    for (index, value) in ['a', 'b'].iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // A function parameter: the pattern &(x, y) destructures the tuple reference.
    print_coordinates(&(3, 5));

    // Try: change the age string from "34" to "twenty". Which branch runs now?
}
