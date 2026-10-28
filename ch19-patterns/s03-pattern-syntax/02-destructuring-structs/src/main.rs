struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // A struct pattern names the fields it wants. Here both fields become variables.
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("x = {x}, y = {y}");

    // A match guard adds an if condition to an arm. The arm runs only when the pattern matches and the guard is true.
    let p2 = Point { x: 5, y: -4 };
    match p2 {
        Point { x: x @ 0..=5, y } if x + y != 0 => {
            println!("on the band, x = {x}, y = {y}");
        }
        Point { x, .. } => println!("other, x = {x}"),
    }

    // Try: change p2 to Point { x: 5, y: -5 }. Which arm runs now, and why?
}
