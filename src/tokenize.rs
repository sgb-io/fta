use std::collections::HashSet;

pub enum TokenType {
    Operator,
    Operand,
}

pub struct Token {
    value: String,
    token_type: TokenType,
}

impl Token {
    pub fn new(value: &str, token_type: TokenType) -> Self {
        Token {
            value: value.to_string(),
            token_type,
        }
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.token_type, TokenType::Operator)
    }

    pub fn is_operand(&self) -> bool {
        matches!(self.token_type, TokenType::Operand)
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut prev_char = '\0';

    for c in code.chars() {
        if is_operator_char(c) || is_whitespace_char(c) {
            if !buffer.is_empty() {
                tokens.push(Token::new(&buffer, TokenType::Operand));
                buffer.clear();
            }
            if is_operator_char(c) && (prev_char != c || (c != '+' && c != '-')) {
                tokens.push(Token::new(&c.to_string(), TokenType::Operator));
            }
        } else {
            buffer.push(c);
        }
        prev_char = c;
    }

    if !buffer.is_empty() {
        tokens.push(Token::new(&buffer, TokenType::Operand));
    }

    tokens
}

fn is_whitespace_char(c: char) -> bool {
    c.is_whitespace()
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
