fn main() {
    let age: u8 = 30;
    let big: i64 = 9_000_000_000;
    let years = 2;
    println!("age {age}, big {big}, years {years}");

    let price = 4.99;
    println!("price {price}");

    let quotient = 17 / 5;
    let remainder = 17 % 5;
    println!("17 / 5 = {quotient}, remainder {remainder}");
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);

    let is_done: bool = false;
    let letter = 'Z';
    let heart_eyed_cat = '😻';
    println!("{is_done} {letter} {heart_eyed_cat}");

    let max: u8 = 255;
    println!("checked: {:?}", max.checked_add(1));
    println!("wrapping: {}", max.wrapping_add(1));
}
