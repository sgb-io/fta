use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Operator(String),
    Operand(String),
}

impl Token {
    pub fn is_operator(&self) -> bool {
        matches!(self, Token::Operator(_))
    }

    pub fn is_operand(&self) -> bool {
        matches!(self, Token::Operand(_))
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.peek() {
        if c.is_alphabetic() || *c == '_' {
            tokens.push(parse_identifier(&mut chars));
        } else if c.is_digit(10) {
            tokens.push(parse_number(&mut chars));
        } else if is_operator_char(*c) {
            tokens.push(parse_operator(&mut chars));
        } else {
            chars.next();
        }
    }

    tokens
}

fn parse_identifier(chars: &mut Peekable<Chars>) -> Token {
    let mut identifier = String::new();

    while let Some(c) = chars.peek() {
        if c.is_alphanumeric() || *c == '_' {
            identifier.push(*c);
            chars.next();
        } else {
            break;
        }
    }

    Token::Operand(identifier)
}

fn parse_number(chars: &mut Peekable<Chars>) -> Token {
    let mut number = String::new();

    while let Some(c) = chars.peek() {
        if c.is_digit(10) {
            number.push(*c);
            chars.next();
        } else {
            break;
        }
    }

    Token::Operand(number)
}

fn parse_operator(chars: &mut Peekable<Chars>) -> Token {
    let operator = chars.next().unwrap().to_string();
    Token::Operator(operator)
}

fn is_operator_char(c: char) -> bool {
    let operators: HashSet<char> = [
        '+', '-', '*', '/', '%', '=', '<', '>', '!', '&', '|', '^', '~', '?', ':', '(', ')', '{',
        '}', '[', ']', ';', ',', '.', '`',
    ]
    .iter()
    .cloned()
    .collect();

    operators.contains(&c)
}

#[test]
fn test_tokenize() {
    let source = "const x: number = 5;";
    let tokens = tokenize(source);
    let expected = vec![
        Token::Operand("const".to_string()),
        Token::Operand("x".to_string()),
        Token::Operator(":".to_string()),
        Token::Operand("number".to_string()),
        Token::Operator("=".to_string()),
        Token::Operand("5".to_string()),
        Token::Operator(";".to_string()),
    ];

    assert_eq!(tokens, expected);
}
