// Calculator, core version: + - * / with precedence, parentheses, unary minus, and a REPL.
use std::fmt;
use std::io::{self, BufRead, Write};

// Tokens are the smallest meaningful pieces of the input.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// The expression tree. Box lets an Expr contain other Exprs.
#[derive(Debug, PartialEq)]
enum Expr {
    Number(f64),
    Negate(Box<Expr>),
    // col is the column of the operator, which is where a division by zero is reported.
    Binary {
        op: Op,
        col: usize,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

// Columns are stored counting from 0, and shown counting from 1.
#[derive(Debug, PartialEq)]
enum CalcError {
    UnexpectedChar { ch: char, col: usize },
    BadNumber { text: String, col: usize },
    Expected { expected: &'static str, found: String, col: usize },
    DivideByZero { col: usize },
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
        }
    }
}

// Turns text into tokens. Each token keeps the column where it starts.
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
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' | '.' => {
                // A number is a run of digits and dots. It's parsed afterwards, so a bad one can be reported.
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
            _ => return Err(CalcError::UnexpectedChar { ch: c, col: i }),
        };
        tokens.push((token, i));
        i += 1;
    }

    Ok(tokens)
}

// Describes a token for an error message.
fn describe(token: &Token) -> String {
    match token {
        Token::Number(n) => n.to_string(),
        Token::Plus => String::from("'+'"),
        Token::Minus => String::from("'-'"),
        Token::Star => String::from("'*'"),
        Token::Slash => String::from("'/'"),
        Token::LParen => String::from("'('"),
        Token::RParen => String::from("')'"),
    }
}

// The parser walks the tokens with a position. Each grammar rule is one method.
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

    // Builds the error for "expected X, but found Y", where Y is the token given, or the end of input.
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

    // factor := '-' factor | number | '(' expr ')'
    fn factor(&mut self) -> Result<Expr, CalcError> {
        match self.peek() {
            Some(Token::Minus) => {
                self.pos += 1;
                Ok(Expr::Negate(Box::new(self.factor()?)))
            }
            Some(Token::Number(n)) => {
                let n = *n;
                self.pos += 1;
                Ok(Expr::Number(n))
            }
            Some(Token::LParen) => {
                self.pos += 1;
                let inner = self.expr()?;
                match self.advance() {
                    Some((Token::RParen, _)) => Ok(inner),
                    found => Err(self.expected("')'", found)),
                }
            }
            _ => Err(self.expected("a number or '('", self.tokens.get(self.pos).cloned())),
        }
    }
}

// Reads a whole expression. Anything left over after it is an error.
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

// Works out the value of a tree.
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
            }
        }
    }
}

// Parses and evaluates one line of input.
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
        "1 / 0",
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

    #[test]
    fn tokenizes_numbers_and_operators() {
        let tokens = tokenize("2 + 3.5").unwrap();
        assert_eq!(tokens, vec![(Token::Number(2.0), 0), (Token::Plus, 2), (Token::Number(3.5), 4)]);
    }

    #[test]
    fn multiplication_binds_tighter_than_addition() {
        assert_eq!(value("2 + 3 * 4"), 14.0);
    }

    #[test]
    fn parentheses_group() {
        assert_eq!(value("(2 + 3) * 4"), 20.0);
    }

    #[test]
    fn operators_of_equal_precedence_go_left_to_right() {
        assert_eq!(value("10 - 4 - 3"), 3.0);
        assert_eq!(value("8 / 4 / 2"), 1.0);
    }

    #[test]
    fn unary_minus_works() {
        assert_eq!(value("-3 + 5"), 2.0);
        assert_eq!(value("2 * -3"), -6.0);
    }

    #[test]
    fn decimals_divide() {
        assert_eq!(value("10 / 4"), 2.5);
        assert_eq!(value("1.5 * 4"), 6.0);
    }

    #[test]
    fn unexpected_character_is_reported_with_its_column() {
        assert_eq!(calculate("2 + $").unwrap_err().to_string(), "unexpected character '$' at column 5");
    }

    #[test]
    fn division_by_zero_is_reported_at_the_operator() {
        assert_eq!(calculate("1 / 0").unwrap_err().to_string(), "division by zero at column 3");
    }

    #[test]
    fn missing_closing_parenthesis_is_reported() {
        assert_eq!(calculate("(2 + 3").unwrap_err().to_string(), "expected ')', found end of input at column 7");
    }

    #[test]
    fn extra_closing_parenthesis_is_reported() {
        assert_eq!(calculate("2 + 3)").unwrap_err().to_string(), "expected end of input, found ')' at column 6");
    }

    #[test]
    fn missing_operand_is_reported() {
        assert_eq!(calculate("2 +").unwrap_err().to_string(), "expected a number or '(', found end of input at column 4");
    }

    #[test]
    fn bad_number_is_reported() {
        assert_eq!(calculate("1.2.3").unwrap_err().to_string(), "invalid number '1.2.3' at column 1");
    }
}
