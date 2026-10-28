#[derive(Debug, PartialEq)]
enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

use Expr::{Add, Mul, Num, Var};

// Simplifies the operands first, and then applies the rules to the pair of results.
fn simplify(expr: Expr) -> Expr {
    match expr {
        Num(n) => Num(n),
        Var(name) => Var(name),
        Add(a, b) => match (simplify(*a), simplify(*b)) {
            (Num(x), Num(y)) => Num(x + y),
            // The or-pattern covers a zero on either side. Both alternatives bind e, with the same type.
            (Num(0), e) | (e, Num(0)) => e,
            (a, b) => Add(Box::new(a), Box::new(b)),
        },
        Mul(a, b) => match (simplify(*a), simplify(*b)) {
            (Num(x), Num(y)) => Num(x * y),
            (Num(0), _) | (_, Num(0)) => Num(0),
            (Num(1), e) | (e, Num(1)) => e,
            (a, b) => Mul(Box::new(a), Box::new(b)),
        },
    }
}

// Prints an expression with parentheses around each operation.
fn show(expr: &Expr) -> String {
    match expr {
        Num(n) => n.to_string(),
        Var(name) => name.clone(),
        Add(a, b) => format!("({} + {})", show(a), show(b)),
        Mul(a, b) => format!("({} * {})", show(a), show(b)),
    }
}

fn main() {
    // (0 + (y * 1))
    let first = Add(
        Box::new(Num(0)),
        Box::new(Mul(Box::new(Var(String::from("y"))), Box::new(Num(1)))),
    );
    println!("before: {}", show(&first));
    println!("after: {}", show(&simplify(first)));

    // ((2 + 3) * x)
    let second = Mul(
        Box::new(Add(Box::new(Num(2)), Box::new(Num(3)))),
        Box::new(Var(String::from("x"))),
    );
    println!("before: {}", show(&second));
    println!("after: {}", show(&simplify(second)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_zero_removes_it() {
        let expr = Add(Box::new(Var(String::from("x"))), Box::new(Num(0)));
        assert_eq!(simplify(expr), Var(String::from("x")));
    }

    #[test]
    fn multiplying_by_zero_gives_zero() {
        let expr = Mul(Box::new(Num(0)), Box::new(Var(String::from("x"))));
        assert_eq!(simplify(expr), Num(0));
    }

    #[test]
    fn two_numbers_are_added() {
        let expr = Add(Box::new(Num(2)), Box::new(Num(3)));
        assert_eq!(simplify(expr), Num(5));
    }
}
