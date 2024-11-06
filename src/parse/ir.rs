use crate::parse::c;

use crate::parse::Identifier;

/// Used for generating unique temporary variable names
static mut IDENTIFIER_COUNT: usize = 0;

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    BitwiseComplement,
    Negation,
}

// TODO: Deriving `Clone` for now to avoid issues with needing to use tmp var AST nodes in
// multiple places, but this should be revisited to see if shared ownership of tmp var AST nodes is
// better
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Constant(u8),
    Var(Identifier),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Return(Value),
    Unary {
        op: UnaryOperator,
        src: Value,
        dst: Value,
    },
}

pub fn parse_unary_operator(node: c::UnaryOperator) -> UnaryOperator {
    match node {
        c::UnaryOperator::BitwiseComplement => UnaryOperator::BitwiseComplement,
        c::UnaryOperator::Negation => UnaryOperator::Negation,
    }
}

pub fn parse_value(node: c::Expression) -> Value {
    match node {
        c::Expression::NumericConstant(val) => Value::Constant(val),
        _ => todo!(),
    }
}

pub fn parse_instruction(node: c::Statement) -> Vec<Instruction> {
    match node {
        c::Statement::Return(exp) => match exp {
            c::Expression::NumericConstant(_) => {
                let value = parse_value(exp);
                vec![Instruction::Return(value)]
            }
            c::Expression::Unary(unop, inner_exp) => {
                let unop_ast_node = parse_unary_operator(unop);
                let value_ast_node = match *inner_exp {
                    c::Expression::NumericConstant(val) => Value::Constant(val),
                    _ => todo!(),
                };
                let tmp_var_ast_node = make_temporary();
                let unop_instruction_ast_node = Instruction::Unary {
                    op: unop_ast_node,
                    src: value_ast_node,
                    dst: tmp_var_ast_node.clone(),
                };
                let return_instruction_ast_node = Instruction::Return(tmp_var_ast_node);
                vec![unop_instruction_ast_node, return_instruction_ast_node]
            }
        },
    }
}

/// Generate an AST node representing a uniquely named temporary variable
fn make_temporary() -> Value {
    let identifier = format!("tmp{}", unsafe { IDENTIFIER_COUNT });
    unsafe { IDENTIFIER_COUNT += 1 };
    Value::Var(identifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_c_constant_to_ir_constant() {
        let value = 2;
        let c_ast_node = c::Expression::NumericConstant(value);
        let expected_ir_ast_node = Value::Constant(value);
        let ir_ast_node = parse_value(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_c_complement_operstor_to_ir_unary_operator() {
        let c_ast_node = c::UnaryOperator::BitwiseComplement;
        let expected_ir_ast_node = UnaryOperator::BitwiseComplement;
        let ir_ast_node = parse_unary_operator(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_c_negation_operator_to_ir_unary_operator() {
        let c_ast_node = c::UnaryOperator::Negation;
        let expected_ir_ast_node = UnaryOperator::Negation;
        let ir_ast_node = parse_unary_operator(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_return_statement_containing_numeric_constant_to_ir_instruction() {
        let value = 2;
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let c_statement_ast_node = c::Statement::Return(c_constant_ast_node);
        let expected_ir_ast_nodes = vec![Instruction::Return(Value::Constant(value))];
        let ir_ast_nodes = parse_instruction(c_statement_ast_node);
        assert_eq!(ir_ast_nodes, expected_ir_ast_nodes);
    }

    #[test]
    fn parse_return_statement_containing_expression_with_one_unary_operator_to_ir_instruction() {
        let value = 2;
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let boxed_expression = Box::new(c_constant_ast_node);
        let c_expression_unary_ast_node =
            c::Expression::Unary(c::UnaryOperator::BitwiseComplement, boxed_expression);
        let c_statement_ast_node = c::Statement::Return(c_expression_unary_ast_node);
        let expected_tmp_var_identifier = "tmp0";
        let expected_ir_instruction_ast_nodes = vec![
            Instruction::Unary {
                op: UnaryOperator::BitwiseComplement,
                src: Value::Constant(value),
                dst: Value::Var(expected_tmp_var_identifier.to_string()),
            },
            Instruction::Return(Value::Var(expected_tmp_var_identifier.to_string())),
        ];
        let ir_ast_nodes = parse_instruction(c_statement_ast_node);
        assert_eq!(ir_ast_nodes, expected_ir_instruction_ast_nodes);
    }
}
