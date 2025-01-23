use crate::parse::c;

use crate::parse::Identifier;

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    BitwiseComplement,
    Negation,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
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
    Binary {
        op: BinaryOperator,
        left: Value,
        right: Value,
        dst: Value,
    },
}

#[derive(Debug, PartialEq)]
pub enum FunctionDefinition {
    Function {
        identifier: String,
        body: Vec<Instruction>,
    },
}

#[derive(Debug, PartialEq)]
pub enum ProgramDefinition {
    Program(FunctionDefinition),
}

pub fn parse_unary_operator(node: c::UnaryOperator) -> UnaryOperator {
    match node {
        c::UnaryOperator::BitwiseComplement => UnaryOperator::BitwiseComplement,
        c::UnaryOperator::Negation => UnaryOperator::Negation,
    }
}

fn parse_binary_operator(node: c::BinaryOperator) -> BinaryOperator {
    match node {
        c::BinaryOperator::Add => BinaryOperator::Add,
        c::BinaryOperator::Subtract => BinaryOperator::Subtract,
        c::BinaryOperator::Multiply => BinaryOperator::Multiply,
        c::BinaryOperator::Divide => BinaryOperator::Divide,
        c::BinaryOperator::Modulo => BinaryOperator::Modulo,
    }
}

pub fn parse_value(node: c::Expression) -> Value {
    match node {
        c::Expression::NumericConstant(val) => Value::Constant(val),
        _ => todo!(),
    }
}

pub fn parse_instruction(node: c::Statement) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut identifier_count: usize = 0;

    match node {
        c::Statement::Return(exp) => {
            let dst = recurse_expression(exp, &mut instructions, &mut identifier_count);
            instructions.push(Instruction::Return(dst));
        }
    }

    instructions
}

fn recurse_expression(
    exp: c::Expression,
    instructions: &mut Vec<Instruction>,
    id: &mut usize,
) -> Value {
    match exp {
        c::Expression::NumericConstant(_) => parse_value(exp),
        c::Expression::Unary(unop, boxed_inner_exp) => {
            let src = recurse_expression(*boxed_inner_exp, instructions, id);
            let dst = make_temporary(id);
            *id += 1;
            let unop_ast_node = parse_unary_operator(unop);
            let unop_instruction_ast_node = Instruction::Unary {
                op: unop_ast_node,
                src,
                dst: dst.clone(),
            };
            instructions.push(unop_instruction_ast_node);
            dst
        }
        c::Expression::Binary { op, left, right } => {
            let left = recurse_expression(*left, instructions, id);
            let right = recurse_expression(*right, instructions, id);
            let dst = make_temporary(id);
            *id += 1;
            let binop_ast_node = parse_binary_operator(op);
            let binop_instruction_ast_node = Instruction::Binary {
                op: binop_ast_node,
                left,
                right,
                dst: dst.clone(),
            };
            instructions.push(binop_instruction_ast_node);
            dst
        }
    }
}

/// Generate an AST node representing a uniquely named temporary variable
fn make_temporary(id: &usize) -> Value {
    let identifier = format!("tmp{}", *id);
    Value::Var(identifier)
}

pub fn parse_function_definition(node: c::FunctionDefinition) -> FunctionDefinition {
    match node {
        c::FunctionDefinition::Function { name, body } => FunctionDefinition::Function {
            identifier: name,
            body: parse_instruction(body),
        },
    }
}

pub fn parse_program_definition(node: c::ProgramDefinition) -> ProgramDefinition {
    match node {
        c::ProgramDefinition::Program(func_defn) => {
            ProgramDefinition::Program(parse_function_definition(func_defn))
        }
    }
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
    fn parse_c_addition_operator_to_ir_binary_operator() {
        let c_ast_node = c::BinaryOperator::Add;
        let expected_ir_ast_node = BinaryOperator::Add;
        let ir_ast_node = parse_binary_operator(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_c_subtraction_operator_to_ir_binary_operator() {
        let c_ast_node = c::BinaryOperator::Subtract;
        let expected_ir_ast_node = BinaryOperator::Subtract;
        let ir_ast_node = parse_binary_operator(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_c_multiplication_operator_to_ir_binary_operator() {
        let c_ast_node = c::BinaryOperator::Multiply;
        let expected_ir_ast_node = BinaryOperator::Multiply;
        let ir_ast_node = parse_binary_operator(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_c_division_operator_to_ir_binary_operator() {
        let c_ast_node = c::BinaryOperator::Divide;
        let expected_ir_ast_node = BinaryOperator::Divide;
        let ir_ast_node = parse_binary_operator(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_c_modulo_operator_to_ir_binary_operator() {
        let c_ast_node = c::BinaryOperator::Modulo;
        let expected_ir_ast_node = BinaryOperator::Modulo;
        let ir_ast_node = parse_binary_operator(c_ast_node);
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

    #[test]
    fn parse_return_statement_containing_expression_with_two_unary_operators_to_ir_instruction() {
        let value = 2;
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let boxed_constant_ast_node = Box::new(c_constant_ast_node);
        let c_inner_unary_ast_node =
            c::Expression::Unary(c::UnaryOperator::BitwiseComplement, boxed_constant_ast_node);
        let boxed_inner_unary_ast_node = Box::new(c_inner_unary_ast_node);
        let c_outer_unary_ast_node =
            c::Expression::Unary(c::UnaryOperator::Negation, boxed_inner_unary_ast_node);
        let c_statement_ast_node = c::Statement::Return(c_outer_unary_ast_node);
        let expected_ir_instruction_ast_nodes = vec![
            Instruction::Unary {
                op: UnaryOperator::BitwiseComplement,
                src: Value::Constant(value),
                dst: Value::Var("tmp0".to_string()),
            },
            Instruction::Unary {
                op: UnaryOperator::Negation,
                src: Value::Var("tmp0".to_string()),
                dst: Value::Var("tmp1".to_string()),
            },
            Instruction::Return(Value::Var("tmp1".to_string())),
        ];
        let ir_ast_nodes = parse_instruction(c_statement_ast_node);
        assert_eq!(ir_ast_nodes, expected_ir_instruction_ast_nodes);
    }

    #[test]
    fn parse_return_statement_containing_expression_with_one_binary_operator_to_ir_instructions() {
        let left_operand = 1;
        let right_operand = 2;
        let c_expression_binary_ast_node = c::Expression::Binary {
            op: c::BinaryOperator::Add,
            left: Box::new(c::Expression::NumericConstant(left_operand)),
            right: Box::new(c::Expression::NumericConstant(right_operand)),
        };
        let c_statement_ast_node = c::Statement::Return(c_expression_binary_ast_node);
        let expected_tmp_var_identifier = "tmp0";
        let expected_ir_instruction_ast_nodes = vec![
            Instruction::Binary {
                op: BinaryOperator::Add,
                left: Value::Constant(1),
                right: Value::Constant(2),
                dst: Value::Var(expected_tmp_var_identifier.to_string()),
            },
            Instruction::Return(Value::Var(expected_tmp_var_identifier.to_string())),
        ];
        let ir_ast_nodes = parse_instruction(c_statement_ast_node);
        assert_eq!(ir_ast_nodes, expected_ir_instruction_ast_nodes);
    }

    #[test]
    fn parse_c_function_defn_to_ir_function_defn() {
        let value = 2;
        let function_identifier = "main";
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let boxed_constant_ast_node = Box::new(c_constant_ast_node);
        let c_inner_unary_ast_node =
            c::Expression::Unary(c::UnaryOperator::BitwiseComplement, boxed_constant_ast_node);
        let boxed_inner_unary_ast_node = Box::new(c_inner_unary_ast_node);
        let c_outer_unary_ast_node =
            c::Expression::Unary(c::UnaryOperator::Negation, boxed_inner_unary_ast_node);
        let c_statement_ast_node = c::Statement::Return(c_outer_unary_ast_node);
        let c_function_defn_ast_node = c::FunctionDefinition::Function {
            name: function_identifier.to_string(),
            body: c_statement_ast_node,
        };
        let ir_instruction_ast_nodes = vec![
            Instruction::Unary {
                op: UnaryOperator::BitwiseComplement,
                src: Value::Constant(value),
                dst: Value::Var("tmp0".to_string()),
            },
            Instruction::Unary {
                op: UnaryOperator::Negation,
                src: Value::Var("tmp0".to_string()),
                dst: Value::Var("tmp1".to_string()),
            },
            Instruction::Return(Value::Var("tmp1".to_string())),
        ];
        let expected_ir_ast_node = FunctionDefinition::Function {
            identifier: function_identifier.to_string(),
            body: ir_instruction_ast_nodes,
        };
        let ir_ast_node = parse_function_definition(c_function_defn_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }

    #[test]
    fn parse_c_program_defn_to_ir_program_defn() {
        let value = 2;
        let function_identifier = "main";
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let boxed_constant_ast_node = Box::new(c_constant_ast_node);
        let c_inner_unary_ast_node =
            c::Expression::Unary(c::UnaryOperator::BitwiseComplement, boxed_constant_ast_node);
        let boxed_inner_unary_ast_node = Box::new(c_inner_unary_ast_node);
        let c_outer_unary_ast_node =
            c::Expression::Unary(c::UnaryOperator::Negation, boxed_inner_unary_ast_node);
        let c_statement_ast_node = c::Statement::Return(c_outer_unary_ast_node);
        let c_function_defn_ast_node = c::FunctionDefinition::Function {
            name: function_identifier.to_string(),
            body: c_statement_ast_node,
        };
        let c_program_defn_ast_node = c::ProgramDefinition::Program(c_function_defn_ast_node);
        let ir_instruction_ast_nodes = vec![
            Instruction::Unary {
                op: UnaryOperator::BitwiseComplement,
                src: Value::Constant(value),
                dst: Value::Var("tmp0".to_string()),
            },
            Instruction::Unary {
                op: UnaryOperator::Negation,
                src: Value::Var("tmp0".to_string()),
                dst: Value::Var("tmp1".to_string()),
            },
            Instruction::Return(Value::Var("tmp1".to_string())),
        ];
        let ir_function_defn_ast_node = FunctionDefinition::Function {
            identifier: function_identifier.to_string(),
            body: ir_instruction_ast_nodes,
        };
        let expected_ir_ast_node = ProgramDefinition::Program(ir_function_defn_ast_node);
        let ir_ast_node = parse_program_definition(c_program_defn_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }
}
