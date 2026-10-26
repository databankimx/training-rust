#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coins = [
        Coin::Penny,
        Coin::Quarter(UsState::Alaska),
        Coin::Nickel,
        Coin::Quarter(UsState::Alabama),
        Coin::Dime,
    ];

    let mut count = 0;
    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {state:?}!");
        } else {
            count += 1;
        }
    }

    println!("Non-quarter coins: {count}");

    // Try: change the else branch to print the coin's position instead of counting. What would you need to track?
}
