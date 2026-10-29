#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // The arms are tried from top to bottom. The first pattern that matches decides the result.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];

    for coin in coins {
        let name = format!("{coin:?}");
        println!("{name} is worth {} cents", value_in_cents(coin));
    }

    // Try: delete the Coin::Dime arm. What does the compiler say?
}
