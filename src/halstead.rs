use std::collections::HashSet;

use crate::tokenize::tokenize;

pub fn calculate(code: &str) -> (usize, usize, usize, usize) {
    let tokens = tokenize(code);
    let mut uniq_operators: HashSet<String> = HashSet::new();
    let mut uniq_operands: HashSet<String> = HashSet::new();
    let mut total_operators = 0;
    let mut total_operands = 0;

    for token in tokens {
        let is_operator = token.is_operator();
        let is_operand = token.is_operand();

        if is_operator {
            uniq_operators.insert(token.value().to_string());
            total_operators += 1;
        } else if is_operand {
            uniq_operands.insert(token.value().to_string());
            total_operands += 1;
        }
    }

    (
        uniq_operators.len(),
        uniq_operands.len(),
        total_operators,
        total_operands,
    )
}
