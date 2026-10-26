fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("first: {}, second: {}, third: {}", tup.0, tup.1, tup.2);
    println!("x is {x}, z is {z}");

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}");
}
