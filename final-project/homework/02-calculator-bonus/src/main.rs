// Calculator, full version. Core features, plus bonus items:
//   1. exponents with ^ (right-associative)
//   2. sqrt(...)
//   3. square and curly brackets, which must match their opening bracket
use std::fmt;
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Number(f64),
    Negate(Box<Expr>),
    Binary {
        op: Op,
        col: usize,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: String,
        col: usize,
        arg: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
enum CalcError {
    UnexpectedChar { ch: char, col: usize },
    BadNumber { text: String, col: usize },
    Expected { expected: &'static str, found: String, col: usize },
    DivideByZero { col: usize },
    UnknownFunction { name: String, col: usize },
    NegativeRoot { col: usize },
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::UnexpectedChar { ch, col } => {
                write!(f, "unexpected character '{ch}' at column {}", col + 1)
            }
            CalcError::BadNumber { text, col } => {
                write!(f, "invalid number '{text}' at column {}", col + 1)
            }
            CalcError::Expected { expected, found, col } => {
                write!(f, "expected {expected}, found {found} at column {}", col + 1)
            }
            CalcError::DivideByZero { col } => {
                write!(f, "division by zero at column {}", col + 1)
            }
            CalcError::UnknownFunction { name, col } => {
                write!(f, "unknown function '{name}' at column {}", col + 1)
            }
            CalcError::NegativeRoot { col } => {
                write!(f, "sqrt of a negative number at column {}", col + 1)
            }
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<(Token, usize)>, CalcError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let token = match c {
            ' ' => {
                i += 1;
                continue;
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '^' => Token::Caret,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '0'..='9' | '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();
                let value = text
                    .parse::<f64>()
                    .map_err(|_| CalcError::BadNumber { text: text.clone(), col: start })?;
                tokens.push((Token::Number(value), start));
                continue;
            }
            c if c.is_ascii_alphabetic() => {
                // A name is a run of letters and digits, such as sqrt.
                let start = i;
                while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                    i += 1;
                }
                let name: String = chars[start..i].iter().collect();
                tokens.push((Token::Ident(name), start));
                continue;
            }
            _ => return Err(CalcError::UnexpectedChar { ch: c, col: i }),
        };
        tokens.push((token, i));
        i += 1;
    }

    Ok(tokens)
}

fn describe(token: &Token) -> String {
    match token {
        Token::Number(n) => n.to_string(),
        Token::Ident(name) => name.clone(),
        Token::Plus => String::from("'+'"),
        Token::Minus => String::from("'-'"),
        Token::Star => String::from("'*'"),
        Token::Slash => String::from("'/'"),
        Token::Caret => String::from("'^'"),
        Token::LParen => String::from("'('"),
        Token::RParen => String::from("')'"),
        Token::LBracket => String::from("'['"),
        Token::RBracket => String::from("']'"),
        Token::LBrace => String::from("'{'"),
        Token::RBrace => String::from("'}'"),
    }
}

// The closing bracket that matches an opening one, as text for an error message.
fn closer_text(closer: &Token) -> &'static str {
    match closer {
        Token::RBracket => "']'",
        Token::RBrace => "'}'",
        _ => "')'",
    }
}

struct Parser {
    tokens: Vec<(Token, usize)>,
    pos: usize,
    end_col: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|(token, _)| token)
    }

    fn advance(&mut self) -> Option<(Token, usize)> {
        let item = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        item
    }

    fn expected(&self, expected: &'static str, found: Option<(Token, usize)>) -> CalcError {
        match found {
            Some((token, col)) => CalcError::Expected { expected, found: describe(&token), col },
            None => CalcError::Expected {
                expected,
                found: String::from("end of input"),
                col: self.end_col,
            },
        }
    }

    // expr := term (('+' | '-') term)*
    fn expr(&mut self) -> Result<Expr, CalcError> {
        let mut left = self.term()?;
        loop {
            let op = match self.peek() {
                Some(Token::Plus) => Op::Add,
                Some(Token::Minus) => Op::Sub,
                _ => break,
            };
            let col = self.tokens[self.pos].1;
            self.pos += 1;
            let right = self.term()?;
            left = Expr::Binary { op, col, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    // term := factor (('*' | '/') factor)*
    fn term(&mut self) -> Result<Expr, CalcError> {
        let mut left = self.factor()?;
        loop {
            let op = match self.peek() {
                Some(Token::Star) => Op::Mul,
                Some(Token::Slash) => Op::Div,
                _ => break,
            };
            let col = self.tokens[self.pos].1;
            self.pos += 1;
            let right = self.factor()?;
            left = Expr::Binary { op, col, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    // factor := '-' factor | power
    // Unary minus is applied after the power, so -2^2 is -(2^2).
    fn factor(&mut self) -> Result<Expr, CalcError> {
        if matches!(self.peek(), Some(Token::Minus)) {
            self.pos += 1;
            return Ok(Expr::Negate(Box::new(self.factor()?)));
        }
        self.power()
    }

    // power := primary ('^' factor)?
    // The right side is a factor, so 2^3^2 groups as 2^(3^2), and 2^-1 is valid.
    fn power(&mut self) -> Result<Expr, CalcError> {
        let base = self.primary()?;
        if matches!(self.peek(), Some(Token::Caret)) {
            let col = self.tokens[self.pos].1;
            self.pos += 1;
            let exponent = self.factor()?;
            return Ok(Expr::Binary {
                op: Op::Pow,
                col,
                left: Box::new(base),
                right: Box::new(exponent),
            });
        }
        Ok(base)
    }

    // primary := number | name '(' expr ')' | opening expr closing
    fn primary(&mut self) -> Result<Expr, CalcError> {
        match self.peek().cloned() {
            Some(Token::Number(n)) => {
                self.pos += 1;
                Ok(Expr::Number(n))
            }
            Some(Token::Ident(name)) => {
                let col = self.tokens[self.pos].1;
                self.pos += 1;
                if name != "sqrt" {
                    return Err(CalcError::UnknownFunction { name, col });
                }
                match self.advance() {
                    Some((Token::LParen, _)) => {}
                    found => return Err(self.expected("'('", found)),
                }
                let arg = self.expr()?;
                match self.advance() {
                    Some((Token::RParen, _)) => {}
                    found => return Err(self.expected("')'", found)),
                }
                Ok(Expr::Call { name, col, arg: Box::new(arg) })
            }
            Some(token @ (Token::LParen | Token::LBracket | Token::LBrace)) => {
                // Each opening bracket has a matching closing one. Any other closing bracket is an error.
                let closer = match token {
                    Token::LParen => Token::RParen,
                    Token::LBracket => Token::RBracket,
                    _ => Token::RBrace,
                };
                self.pos += 1;
                let inner = self.expr()?;
                match self.advance() {
                    Some((found, _)) if found == closer => Ok(inner),
                    found => Err(self.expected(closer_text(&closer), found)),
                }
            }
            _ => Err(self.expected("a number or '('", self.tokens.get(self.pos).cloned())),
        }
    }
}

fn parse(input: &str) -> Result<Expr, CalcError> {
    let tokens = tokenize(input)?;
    let end_col = input.chars().count();
    let mut parser = Parser { tokens, pos: 0, end_col };

    let expr = parser.expr()?;
    if parser.pos < parser.tokens.len() {
        return Err(parser.expected("end of input", parser.tokens.get(parser.pos).cloned()));
    }
    Ok(expr)
}

fn evaluate(expr: &Expr) -> Result<f64, CalcError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Negate(inner) => Ok(-evaluate(inner)?),
        Expr::Binary { op, col, left, right } => {
            let a = evaluate(left)?;
            let b = evaluate(right)?;
            match op {
                Op::Add => Ok(a + b),
                Op::Sub => Ok(a - b),
                Op::Mul => Ok(a * b),
                Op::Div => {
                    if b == 0.0 {
                        Err(CalcError::DivideByZero { col: *col })
                    } else {
                        Ok(a / b)
                    }
                }
                Op::Pow => Ok(a.powf(b)),
            }
        }
        Expr::Call { name, col, arg } => {
            let value = evaluate(arg)?;
            match name.as_str() {
                "sqrt" => {
                    if value < 0.0 {
                        Err(CalcError::NegativeRoot { col: *col })
                    } else {
                        Ok(value.sqrt())
                    }
                }
                _ => Err(CalcError::UnknownFunction { name: name.clone(), col: *col }),
            }
        }
    }
}

fn calculate(input: &str) -> Result<f64, CalcError> {
    evaluate(&parse(input)?)
}

// Runs the demo set when the program is started with `demo`, printing each expression with its result or error.
fn run_demo() {
    let expressions = [
        "2 + 3 * 4",
        "(2 + 3) * 4",
        "10 - 4 - 3",
        "8 / 4 / 2",
        "-3 + 5",
        "2 * -3",
        "10 / 4",
        "1.5 * 4",
        "2^3^2",
        "2 * 3^2",
        "-2^2",
        "2^-1",
        "sqrt(16)",
        "sqrt(-1)",
        "foo(2)",
        "8^(1/3)",
        "[1 + 1] * 2",
        "{2 * 3}",
        "1 / 0",
        "(2 + 3]",
        "(2 + 3",
        "2 + 3)",
        "2 +",
        "1.2.3",
        "2 + $",
    ];
    for expression in expressions {
        match calculate(expression) {
            Ok(value) => println!("{expression} = {value}"),
            Err(error) => println!("{expression} -> error: {error}"),
        }
    }
}

fn main() {
    // cargo run -- demo runs the demo set and exits.
    if std::env::args().any(|arg| arg == "demo") {
        run_demo();
        return;
    }

    let stdin = io::stdin();
    println!("Calculator. Type an expression, or 'quit' to exit.");
    println!("Bonus items included: ^ exponents, sqrt(...), and [ ] and {{ }} brackets.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break;
        }
        let line = line.trim();

        if line.is_empty() {
            continue;
        }
        if line == "quit" {
            break;
        }

        match calculate(line) {
            Ok(value) => println!("{value}"),
            Err(error) => println!("error: {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(input: &str) -> f64 {
        calculate(input).unwrap()
    }

    fn message(input: &str) -> String {
        calculate(input).unwrap_err().to_string()
    }

    #[test]
    fn core_examples_still_work() {
        assert_eq!(value("2 + 3 * 4"), 14.0);
        assert_eq!(value("(2 + 3) * 4"), 20.0);
        assert_eq!(value("10 - 4 - 3"), 3.0);
        assert_eq!(value("8 / 4 / 2"), 1.0);
        assert_eq!(value("-3 + 5"), 2.0);
        assert_eq!(value("2 * -3"), -6.0);
        assert_eq!(value("10 / 4"), 2.5);
    }

    #[test]
    fn exponents_bind_tighter_than_multiplication() {
        assert_eq!(value("2 * 3^2"), 18.0);
    }

    #[test]
    fn exponents_are_right_associative() {
        assert_eq!(value("2^3^2"), 512.0);
    }

    #[test]
    fn unary_minus_applies_after_the_power() {
        assert_eq!(value("-2^2"), -4.0);
    }

    #[test]
    fn exponent_can_be_negative() {
        assert_eq!(value("2^-1"), 0.5);
    }

    #[test]
    fn sqrt_of_a_square() {
        assert_eq!(value("sqrt(16)"), 4.0);
        assert!((value("sqrt(2) * sqrt(2)") - 2.0).abs() < 1e-9);
    }

    #[test]
    fn cube_root_by_exponent() {
        assert!((value("8^(1/3)") - 2.0).abs() < 1e-9);
    }

    #[test]
    fn sqrt_of_a_negative_is_an_error() {
        assert_eq!(message("sqrt(-1)"), "sqrt of a negative number at column 1");
    }

    #[test]
    fn unknown_function_is_an_error() {
        assert_eq!(message("foo(2)"), "unknown function 'foo' at column 1");
    }

    #[test]
    fn square_and_curly_brackets_group() {
        assert_eq!(value("[1 + 1] * 2"), 4.0);
        assert_eq!(value("{2 * 3}"), 6.0);
    }

    #[test]
    fn mismatched_bracket_is_an_error() {
        assert_eq!(message("(2 + 3]"), "expected ')', found ']' at column 7");
    }

    #[test]
    fn division_by_zero_is_still_reported() {
        assert_eq!(message("1 / 0"), "division by zero at column 3");
    }
}
