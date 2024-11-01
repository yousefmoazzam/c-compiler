use std::collections::VecDeque;

use crate::lex::Token;

type Identifier = String;

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumericConstant(u8),
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
}

pub mod asm {

    #[derive(Debug, PartialEq)]
    pub enum Operand {
        Imm(u8),
        // NOTE: Only need to use a single register (`EAX`) as an instruction operand so far, so
        // there's no need to be able to specify which register is being used. Thus, for now,
        // always assume this variant to mean the `EAX` register.
        Register,
    }

    #[derive(Debug, PartialEq)]
    pub enum Instruction {
        Mov { src: Operand, dst: Operand },
        Ret,
    }

    #[derive(Debug, PartialEq)]
    pub enum FunctionDefinition {
        Function {
            name: crate::parse::Identifier,
            instructions: Vec<Instruction>,
        },
    }

    #[derive(Debug, PartialEq)]
    pub enum ProgramDefinition {
        Program(FunctionDefinition),
    }

    pub fn parse_operand(node: crate::parse::Expression) -> Operand {
        match node {
            crate::parse::Expression::NumericConstant(val) => Operand::Imm(val),
        }
    }

    pub fn parse_instructions(node: crate::parse::Statement) -> Vec<Instruction> {
        match node {
            crate::parse::Statement::Return(exp) => {
                let src = parse_operand(exp);
                let dst = Operand::Register;
                vec![Instruction::Mov { src: src, dst: dst }]
            }
        }
    }

    pub fn parse_function_definition(node: crate::parse::FunctionDefinition) -> FunctionDefinition {
        match node {
            crate::parse::FunctionDefinition::Function { name, body } => {
                let instructions = parse_instructions(body);
                FunctionDefinition::Function { name, instructions }
            }
        }
    }

    pub fn parse_program_definition(node: crate::parse::ProgramDefinition) -> ProgramDefinition {
        match node {
            crate::parse::ProgramDefinition::Program(c_func_defn) => {
                let asm_function_definition = parse_function_definition(c_func_defn);
                ProgramDefinition::Program(asm_function_definition)
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn parse_c_constant_to_asm_immediate() {
            let value = 2;
            let c_ast_node = crate::parse::Expression::NumericConstant(value);
            let expected_asm_ast_node = Operand::Imm(value);
            let asm_ast_node = parse_operand(c_ast_node);
            assert_eq!(asm_ast_node, expected_asm_ast_node);
        }

        #[test]
        fn parse_c_return_to_asm_instructions() {
            let value = 2;
            let c_constant_ast_node = crate::parse::Expression::NumericConstant(value);
            let c_return_ast_node = crate::parse::Statement::Return(c_constant_ast_node);
            let expected_asm_ast_instruction_nodes = vec![Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register,
            }];
            let asm_ast_instruction_nodes = parse_instructions(c_return_ast_node);
            assert_eq!(
                asm_ast_instruction_nodes,
                expected_asm_ast_instruction_nodes
            );
        }

        #[test]
        fn parse_c_function_defn_to_asm_function_defn() {
            let value = 2;
            let identifier = "main";
            let c_constant_ast_node = crate::parse::Expression::NumericConstant(value);
            let c_return_ast_node = crate::parse::Statement::Return(c_constant_ast_node);
            let c_function_defn_ast_node = crate::parse::FunctionDefinition::Function {
                name: identifier.to_string(),
                body: c_return_ast_node,
            };
            let expected_asm_instructions = vec![Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register,
            }];
            let expected_asm_ast_node = FunctionDefinition::Function {
                name: identifier.to_string(),
                instructions: expected_asm_instructions,
            };
            let asm_ast_node = parse_function_definition(c_function_defn_ast_node);
            assert_eq!(asm_ast_node, expected_asm_ast_node);
        }

        #[test]
        fn parse_c_program_definition_to_asm_program_defn() {
            let value = 2;
            let identifier = "main";
            let c_constant_ast_node = crate::parse::Expression::NumericConstant(value);
            let c_return_ast_node = crate::parse::Statement::Return(c_constant_ast_node);
            let c_function_defn_ast_node = crate::parse::FunctionDefinition::Function {
                name: identifier.to_string(),
                body: c_return_ast_node,
            };
            let c_program_defn_ast_node =
                crate::parse::ProgramDefinition::Program(c_function_defn_ast_node);
            let asm_instructions = vec![Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register,
            }];
            let asm_function_defn_ast_node = FunctionDefinition::Function {
                name: identifier.to_string(),
                instructions: asm_instructions,
            };
            let expected_asm_ast_node = ProgramDefinition::Program(asm_function_defn_ast_node);
            let asm_ast_node = parse_program_definition(c_program_defn_ast_node);
            assert_eq!(asm_ast_node, expected_asm_ast_node);
        }
    }
}
