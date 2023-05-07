use crate::tokenize::tokenize;

pub fn calculate(code: &str) -> (usize, usize, usize, usize) {
    let tokens = tokenize(code);
    let mut uniq_operators = 0;
    let mut uniq_operands = 0;
    let mut total_operators = 0;
    let mut total_operands = 0;
    let mut prev_token_was_operator = false;

    for token in tokens {
        let is_operator = token.is_operator();
        let is_operand = token.is_operand();
        if is_operator {
            if !prev_token_was_operator {
                uniq_operators += 1;
            }
            uniq_operands += 1;
            prev_token_was_operator = true;
        } else if is_operand {
            if prev_token_was_operator {
                uniq_operands += 1;
            }
            total_operands += 1;
            prev_token_was_operator = false;
        } else {
            total_operators += 1;
            prev_token_was_operator = false;
        }
    }

    (
        uniq_operators,
        uniq_operands,
        total_operators,
        total_operands,
    )
}
