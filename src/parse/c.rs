use std::collections::VecDeque;

use crate::lex::Token;
use crate::parse::Identifier;

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    BitwiseComplement,
    Negation,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumericConstant(u8),
    Unary(UnaryOperator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum FunctionDefinition {
    Function { name: Identifier, body: Statement },
}

#[derive(Debug, PartialEq)]
pub enum ProgramDefinition {
    Program(FunctionDefinition),
}

pub fn parse_unary_operator(tokens: &mut VecDeque<Token>) -> UnaryOperator {
    let next_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");

    match next_token {
        Token::BitwiseComplementOperator => UnaryOperator::BitwiseComplement,
        Token::NegationOperator => UnaryOperator::Negation,
        _ => todo!(),
    }
}

pub fn parse_expression(tokens: &mut VecDeque<Token>) -> Expression {
    // The queue of tokens shouldn't be empty if the queue has been handled correctly by others, so
    // the panic shouldn't occur. Hence, the use of `expect()`.
    let next_token = tokens
        .front()
        .expect("Should have non-empty queue of tokens");

    match next_token {
        Token::NumericConstant(_) => {
            // NOTE: Not able to use the value inside the token since that's an immutable reference
            // to the value, and we also need to consume the token (via popping it off the queue).
            //
            // The borrow checker won't allow the use of the value inside the token reference if a
            // pop happens before it (due to it involving a mutation of `tokens`). Instead have to
            // ignore the value in the token reference and use the value in the popped token.
            let token = tokens
                .pop_front()
                .expect("Already confirmed at least one token in the queue");

            // TODO: It's clunky to have to match against the variant again for the popped token,
            // even though from being inside this match arm we know that the variant must be
            // `Token::NumericConstant`.
            //
            // Find a nicer way to do this.
            match token {
                Token::NumericConstant(val) => Expression::NumericConstant(val),
                _ => panic!(),
            }
        }
        Token::BitwiseComplementOperator | Token::NegationOperator => {
            let unary_operator_ast_node = parse_unary_operator(tokens);
            let inner_expression_ast_node = parse_expression(tokens);
            Expression::Unary(unary_operator_ast_node, Box::new(inner_expression_ast_node))
        }
        Token::OpenParenthesis => {
            _ = tokens
                .pop_front()
                .expect("Already confirmed at least one token in the queue");

            let expression_ast_node = parse_expression(tokens);

            let trailing_token = tokens
                .pop_front()
                .expect("Should be a close parenthesis token for valid syntax");
            if let Token::CloseParenthesis = trailing_token {
                return expression_ast_node;
            }

            // If execution has reached here then the token after the open parenthesis + expression
            // was not a close parenthesis token, which means that the C source code has invalid
            // syntax.
            panic!("Invalid syntax: expected closing parenthesis");
        }
        _ => todo!(),
    }
}

pub fn parse_statement(tokens: &mut VecDeque<Token>) -> Statement {
    // The queue of tokens shouldn't be empty if the queue has been handled correctly by others, so
    // the panic shouldn't occur. Hence, the use of `expect()`.
    let first_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
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

pub fn parse_function_definition(tokens: &mut VecDeque<Token>) -> FunctionDefinition {
    let next_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    if next_token != Token::IntKeyword {
        todo!()
    }

    let next_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    let identifier = match next_token {
        Token::Identifier(identifier) => identifier,
        _ => todo!(),
    };

    let next_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    if next_token != Token::OpenParenthesis {
        todo!()
    }

    let next_token = tokens
        .pop_front()
        .expect("Should have non-emtyp queue of tokens");
    if next_token != Token::CloseParenthesis {
        todo!()
    }

    let next_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    if next_token != Token::OpenBrace {
        todo!()
    }

    let statement_ast_node = parse_statement(tokens);

    let next_token = tokens
        .pop_front()
        .expect("Should have non-empty queue of tokens");
    if next_token != Token::CloseBrace {
        todo!()
    }

    FunctionDefinition::Function {
        name: identifier.to_string(),
        body: statement_ast_node,
    }
}

pub fn parse_program_definition(tokens: &mut VecDeque<Token>) -> ProgramDefinition {
    let function_defn_ast_node = parse_function_definition(tokens);
    ProgramDefinition::Program(function_defn_ast_node)
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
    fn parse_expression_containing_bitwise_complement_operator() {
        let value = 2;
        let mut tokens = VecDeque::from([
            Token::BitwiseComplementOperator,
            Token::NumericConstant(value),
        ]);
        let boxed_expression_ast_node = Box::new(Expression::NumericConstant(value));
        let expected_ast_node =
            Expression::Unary(UnaryOperator::BitwiseComplement, boxed_expression_ast_node);
        let ast_node = parse_expression(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(ast_node, expected_ast_node);
    }

    #[test]
    fn parse_expression_containing_negation_operator() {
        let value = 2;
        let mut tokens = VecDeque::from([Token::NegationOperator, Token::NumericConstant(value)]);
        let boxed_expression_ast_node = Box::new(Expression::NumericConstant(value));
        let expected_ast_node =
            Expression::Unary(UnaryOperator::Negation, boxed_expression_ast_node);
        let ast_node = parse_expression(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(ast_node, expected_ast_node);
    }

    #[test]
    fn parse_expression_containing_expression_wrapped_in_parentheses() {
        let value = 2;
        let mut tokens = VecDeque::from([
            Token::OpenParenthesis,
            Token::NegationOperator,
            Token::NumericConstant(value),
            Token::CloseParenthesis,
        ]);
        let boxed_expression_ast_node = Box::new(Expression::NumericConstant(value));
        let expected_ast_node =
            Expression::Unary(UnaryOperator::Negation, boxed_expression_ast_node);
        let ast_node = parse_expression(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(ast_node, expected_ast_node);
    }

    #[test]
    #[should_panic(expected = "Invalid syntax: expected closing parenthesis")]
    fn panic_if_open_parenthesis_before_expression_but_no_close_parenthesis_after() {
        let value = 2;
        let mut tokens = VecDeque::from([
            Token::OpenParenthesis,
            Token::NegationOperator,
            Token::NumericConstant(value),
            Token::CloseBrace,
        ]);
        _ = parse_expression(&mut tokens);
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

    #[test]
    fn parse_function_defn_with_int_return_and_statement_as_body() {
        let value = 2;
        let identifier = "main";
        let mut tokens = VecDeque::from([
            Token::IntKeyword,
            Token::Identifier(identifier.to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::NumericConstant(value),
            Token::Semicolon,
            Token::CloseBrace,
        ]);
        let expression_ast_node = Expression::NumericConstant(value);
        let statement_ast_node = Statement::Return(expression_ast_node);
        let expected_ast_node = FunctionDefinition::Function {
            name: identifier.to_string(),
            body: statement_ast_node,
        };
        let ast_node = parse_function_definition(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(ast_node, expected_ast_node);
    }

    #[test]
    fn parse_program_defn_consisting_of_single_function_defn() {
        let value = 2;
        let identifier = "main";
        let mut tokens = VecDeque::from([
            Token::IntKeyword,
            Token::Identifier(identifier.to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::NumericConstant(value),
            Token::Semicolon,
            Token::CloseBrace,
        ]);
        let expression_ast_node = Expression::NumericConstant(value);
        let statement_ast_node = Statement::Return(expression_ast_node);
        let function_defn_ast_node = FunctionDefinition::Function {
            name: identifier.to_string(),
            body: statement_ast_node,
        };
        let expected_ast_node = ProgramDefinition::Program(function_defn_ast_node);
        let ast_node = parse_program_definition(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(ast_node, expected_ast_node);
    }

    #[test]
    fn parse_bitwise_complement_operator() {
        let mut tokens = VecDeque::from([Token::BitwiseComplementOperator]);
        let expected_ast_node = UnaryOperator::BitwiseComplement;
        let ast_node = parse_unary_operator(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(expected_ast_node, ast_node);
    }

    #[test]
    fn parse_negation_operator() {
        let mut tokens = VecDeque::from([Token::NegationOperator]);
        let expected_ast_node = UnaryOperator::Negation;
        let ast_node = parse_unary_operator(&mut tokens);
        assert_eq!(0, tokens.len());
        assert_eq!(expected_ast_node, ast_node);
    }
}