use std::collections::VecDeque;

use crate::lex::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumericConstant(u8),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expression),
}

pub fn parse_expression(tokens: &mut VecDeque<Token>) -> Expression {
    // The queue of tokens shouldn't be empty if the queue has been handled correctly by others, so
    // the panic shouldn't occur. Hence, the use of `expect()`.
    let next_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    if let Token::NumericConstant(val) = next_token {
        return Expression::NumericConstant(val);
    }

    // TODO: Handle if the token isn't an expected variant
    todo!()
}

pub fn parse_statement(tokens: &mut VecDeque<Token>) -> Statement {
    // The queue of tokens shouldn't be empty if the queue has been handled correctly by others, so
    // the panic shouldn't occur. Hence, the use of `expect()`.
    let first_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    println!("first_token is {:?}", first_token);
    if first_token != Token::ReturnKeyword {
        todo!()
    }

    let expression_ast_node = parse_expression(tokens);

    let third_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    if third_token != Token::Semicolon {
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
        let mut tokens = VecDeque::from([Token::NumericConstant(value)]);
        let expected_ast_node = Expression::NumericConstant(value);
        let ast_node = parse_expression(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(ast_node, expected_ast_node);
    }

    #[test]
    fn parse_statement_with_return_identifier_and_numeric_expression() {
        let value = 2;
        let mut tokens = VecDeque::from([
            Token::ReturnKeyword,
            Token::NumericConstant(value),
            Token::Semicolon,
        ]);
        let expected_ast_node = Statement::Return(Expression::NumericConstant(value));
        let ast_node = parse_statement(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(ast_node, expected_ast_node);
    }
}
