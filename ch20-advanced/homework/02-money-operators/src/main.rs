use std::fmt;
use std::ops::{Add, Mul, Sub};

// Money is kept in cents, so amounts never lose a fraction of a cent.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cents(i64);

// Rhs defaults to Self, so this is Add<Cents> for Cents.
impl Add for Cents {
    type Output = Cents;

    fn add(self, other: Cents) -> Cents {
        Cents(self.0 + other.0)
    }
}

impl Sub for Cents {
    type Output = Cents;

    fn sub(self, other: Cents) -> Cents {
        Cents(self.0 - other.0)
    }
}

// Multiplying by a count is not a Cents value, so Rhs is set to i64.
impl Mul<i64> for Cents {
    type Output = Cents;

    fn mul(self, count: i64) -> Cents {
        Cents(self.0 * count)
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.0 / 100, self.0 % 100)
    }
}

fn main() {
    let total = Cents(1250) * 3 + Cents(99);
    println!("total: {total}");

    let change = Cents(5000) - total;
    println!("change: {change}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_amounts() {
        assert_eq!(Cents(150) + Cents(75), Cents(225));
    }

    #[test]
    fn subtracts_two_amounts() {
        assert_eq!(Cents(500) - Cents(120), Cents(380));
    }

    #[test]
    fn multiplies_by_a_count() {
        assert_eq!(Cents(250) * 4, Cents(1000));
    }

    #[test]
    fn displays_as_dollars_and_cents() {
        assert_eq!(Cents(5).to_string(), "$0.05");
    }
}
