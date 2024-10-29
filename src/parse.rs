use crate::lex::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumericConstant(u8),
}

pub fn parse_expression(tokens: &[Token]) -> Expression {
    let next_token = &tokens[0];
    if let Token::NumericConstant(val) = next_token {
        return Expression::NumericConstant(*val);
    }

    // TODO: Handle if the token isn't an expected variant
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expression_containing_numeric_constant() {
        let value = 2;
        let tokens = vec![Token::NumericConstant(value)];
        let expected_ast_node = Expression::NumericConstant(value);
        let ast_node = parse_expression(&tokens[..]);
        assert_eq!(ast_node, expected_ast_node);
    }
}
