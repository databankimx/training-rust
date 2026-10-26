struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2 };

    // Rust tracks fields separately, so p.x and p.y can be borrowed mutably at the same time.
    let x = &mut p.x;
    let y = &mut p.y;
    *x += 1;
    *y += 1;

    println!("{} {}", p.x, p.y);

    // Try: call a function that takes &Point, such as print_point(&p), between the borrow of p.x and the
    // line that uses x. Why does Rust reject it, even though it only reads the point?
}
