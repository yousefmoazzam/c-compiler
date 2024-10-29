use crate::lex::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumericConstant(u8),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expression),
}

pub fn parse_expression(tokens: &[Token]) -> Expression {
    let next_token = &tokens[0];
    if let Token::NumericConstant(val) = next_token {
        return Expression::NumericConstant(*val);
    }

    // TODO: Handle if the token isn't an expected variant
    todo!()
}

pub fn parse_statement(tokens: &[Token]) -> Statement {
    let first_token = &tokens[0];
    if *first_token != Token::ReturnKeyword {
        todo!()
    }

    let expression_ast_node = parse_expression(&tokens[1..]);

    let third_token = &tokens[2];
    if *third_token != Token::Semicolon {
        todo!()
    }

    Statement::Return(expression_ast_node)
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

    #[test]
    fn parse_statement_with_return_identifier_and_numeric_expression() {
        let value = 2;
        let tokens = vec![
            Token::ReturnKeyword,
            Token::NumericConstant(value),
            Token::Semicolon,
        ];
        let expected_ast_node = Statement::Return(Expression::NumericConstant(value));
        let ast_node = parse_statement(&tokens[..]);
        assert_eq!(ast_node, expected_ast_node);
    }
}
