// Academic implementation of evaluation of an expression.
// Evaluation is performed in three major steps:
// 1. Tokenization.
// 2. Transformation from infix to postfix form.
// 3. Evaluation of postfix form.
// Inspired by those tutorials:
// Infix to Postfix using stack: https://www.youtube.com/watch?v=vq-nUF0G4fI
// Evaluation of Prefix and Postfix expressions using stack: https://www.youtube.com/watch?v=MeRb_1bddWg

use self::Parenthesis::*;
use self::Operator::*;

pub fn calc(expression: &str) -> f64 {
    evaluate_postfix_expression(convert_to_postfix_notation(split_into_tokens(expression)))
}

trait TryParse<T>: Sized {
    fn try_parse(_: T) -> Option<Self>;
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Scalar(Number),
    Operator(Operator),
    Parenthesis(Parenthesis),
    UnaryMinus
}

impl Token {
    pub fn is_number(&self) -> bool {
        match self {
            Token::Scalar(..) => true,
            _ => false,
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Operator(..) => true,
            _ => false,
        }
    }

    pub fn is_parenthesis(&self) -> bool {
        match self {
            Token::Parenthesis(..) => true,
            _ => false,
        }
    }

    pub fn is_unary_minus(&self) -> bool {
        match self {
            Token::UnaryMinus => true,
            _ => false,
        }
    }

    pub fn is_opening_parenthesis(&self) -> bool {
        match self {
            Token::Parenthesis(Parenthesis::Opening) => true,
            _ => false,
        }
    }

    pub fn is_closing_parenthesis(&self) -> bool {
        match self {
            Token::Parenthesis(Parenthesis::Closing) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Number(f64);

impl Number {
    pub fn apply_unary_minus(&self) -> Number {
        Number(-self.0)
    }
}

impl TryParse<&str> for Number {
    fn try_parse(num_str: &str) -> Option<Self> {
        match num_str.parse::<f64>() {
            Ok(num) => Some(Self(num)),
            Err(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn is_sub(&self) -> bool {
        match self {
            Self::Sub => true,
            _ => false,
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div => 2,
        }
    }

    pub fn apply(&self, num_1: Number, num_2: Number) -> Number {
        let a = num_1.0;
        let b = num_2.0;
        Number(match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        })
    }
}

impl TryParse<&char> for Operator {
    fn try_parse(c: &char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Parenthesis {
    Opening,
    Closing
}

impl Parenthesis {
    pub fn is_opening(&self) -> bool {
        match self {
            Parenthesis::Opening => true,
            _ => false,
        }
    }
}

impl TryParse<&char> for Parenthesis {
    fn try_parse(c: &char) -> Option<Self> {
        match c {
            '(' => Some(Self::Opening),
            ')' => Some(Self::Closing),
            _ => None,
        }
    }
}

fn evaluate_postfix_expression(tokens: Vec<Token>) -> f64 {
    let mut arg_stack = Vec::with_capacity(tokens.len());
    for token in tokens {
        if token.is_number() {
            arg_stack.push(token);
        } else {
            if token.is_unary_minus() {
                // Apply unary minus for the last argument.
                if let Token::Scalar(num) = arg_stack.pop().unwrap() {
                    arg_stack.push(Token::Scalar(num.apply_unary_minus()));
                }
            } else {
                match (token, arg_stack.pop().unwrap(), arg_stack.pop().unwrap()) {
                    (
                        Token::Operator(op),
                        Token::Scalar(num_2),
                        Token::Scalar(num_1),
                    ) => arg_stack.push(Token::Scalar(op.apply(num_1, num_2))),
                    _ => (),
                }
            }
        }
    }
    if let Token::Scalar(num) = arg_stack.pop().unwrap() {
        return num.0;
    }
    panic!("Unexpected end of the argument stack");
}

fn convert_to_postfix_notation(tokens: Vec<Token>) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut op_stack: Vec<Token> = Vec::with_capacity(32);
    for token in tokens.into_iter() {
        if token.is_number() {
            res.push(token);
        } else if token.is_unary_minus() {
            op_stack.push(token);
        } else if token.is_opening_parenthesis() {
            op_stack.push(token);
        } else if token.is_closing_parenthesis() {
            // Assume we have corresponding opening parenthesis.
            let mut last_op = op_stack.pop().unwrap();
            while !last_op.is_opening_parenthesis() {
                res.push(last_op);
                last_op = op_stack.pop().unwrap();
            };
        } else if let Token::Operator(cur_op) = token {
            if !op_stack.is_empty() {
                let mut last_token = op_stack.last().unwrap().clone();
                while !op_stack.is_empty() && !last_token.is_opening_parenthesis() {
                    if let Token::Operator(last_op) = last_token {
                        if last_op.precedence() < cur_op.precedence() { break; }
                    }
                    res.push(last_token);
                    last_token = op_stack.pop().unwrap();
                }
            }
            op_stack.push(token);
        }
    }
    while !op_stack.is_empty() {
        res.push(op_stack.pop().unwrap());
    }
    res
}

fn split_into_tokens(expression: &str) -> Vec<Token> {
    let dec_sep = '.';
    let mut res: Vec<Token> = Vec::with_capacity(expression.len());
    let mut digits: Vec<char> = Vec::with_capacity(8);

    for c in expression.chars() {
        if c.is_ascii_digit() || c == dec_sep {
            digits.push(c);
        } else {
            if !digits.is_empty() {
                res.push(parse_number(&digits));
                digits.clear();
            }
            if let Some(op) = Operator::try_parse(&c) {
                if op.is_sub() {
                    if let Some(last_token) = res.last() {
                        // Assume we can have unary minus after an operator.
                        if last_token.is_operator()
                            || last_token.is_parenthesis() {
                            res.push(Token::UnaryMinus);
                        } else {
                            res.push(Token::Operator(op));
                        }
                    } else {
                        res.push(Token::UnaryMinus);
                    }
                } else {
                    res.push(Token::Operator(op));
                }
            } else if let Some(par) = Parenthesis::try_parse(&c) {
                res.push(Token::Parenthesis(par));
            } else if !c.is_ascii_whitespace() {
                panic!("Unexpected symbol: '{}'", c);
            }
        }
    }
    if !digits.is_empty() {
        res.push(parse_number(&digits));
    }
    res
}

fn parse_number(digits: &[char]) -> Token {
    let num_str: String = digits.iter().collect();
    if let Some(num) = Number::try_parse(&num_str) {
        return Token::Scalar(num);
    }
    panic!(format!("Failed to parse number: '{}'", num_str));
}
