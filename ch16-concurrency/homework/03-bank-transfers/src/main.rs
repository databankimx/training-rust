use std::sync::{Arc, Mutex};
use std::thread;

// Moves amount from one account to another. The two locks are always taken lower index first, so no two transfers can wait for each other.
fn transfer(accounts: &[Mutex<i64>], from: usize, to: usize, amount: i64) {
    let (first, second) = if from < to { (from, to) } else { (to, from) };
    let mut first_guard = accounts[first].lock().unwrap();
    let mut second_guard = accounts[second].lock().unwrap();

    // Work out which guard is the source and which is the destination, since they were taken in index order.
    let (source, destination) = if from < to {
        (&mut first_guard, &mut second_guard)
    } else {
        (&mut second_guard, &mut first_guard)
    };
    **source -= amount;
    **destination += amount;
}

fn main() {
    let accounts = Arc::new(vec![Mutex::new(1000_i64), Mutex::new(1000_i64)]);
    let mut handles = Vec::new();

    for _ in 0..5 {
        let accounts = Arc::clone(&accounts);
        handles.push(thread::spawn(move || transfer(&accounts, 0, 1, 100)));
    }
    for _ in 0..5 {
        let accounts = Arc::clone(&accounts);
        handles.push(thread::spawn(move || transfer(&accounts, 1, 0, 50)));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let balances: Vec<i64> = accounts.iter().map(|account| *account.lock().unwrap()).collect();
    println!("balances: {balances:?}");
    println!("total: {}", balances.iter().sum::<i64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_transfer_moves_the_money() {
        let accounts = vec![Mutex::new(100_i64), Mutex::new(0_i64)];
        transfer(&accounts, 0, 1, 30);
        assert_eq!(*accounts[0].lock().unwrap(), 70);
        assert_eq!(*accounts[1].lock().unwrap(), 30);
    }

    #[test]
    fn a_transfer_keeps_the_total() {
        let accounts = vec![Mutex::new(100_i64), Mutex::new(50_i64)];
        transfer(&accounts, 1, 0, 20);
        let total: i64 = accounts.iter().map(|a| *a.lock().unwrap()).sum();
        assert_eq!(total, 150);
    }
}
