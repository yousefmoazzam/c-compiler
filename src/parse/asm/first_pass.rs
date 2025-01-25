use crate::parse::asm::{
    BinaryOperator, FunctionDefinition, Instruction, Operand, ProgramDefinition, Reg, UnaryOperator,
};
use crate::parse::ir;

pub fn parse_unary_operator(node: ir::UnaryOperator) -> UnaryOperator {
    match node {
        ir::UnaryOperator::BitwiseComplement => UnaryOperator::Not,
        ir::UnaryOperator::Negation => UnaryOperator::Neg,
    }
}

fn parse_binary_operator(node: ir::BinaryOperator) -> BinaryOperator {
    match node {
        ir::BinaryOperator::Add => BinaryOperator::Add,
        ir::BinaryOperator::Subtract => BinaryOperator::Subtract,
        ir::BinaryOperator::Multiply => BinaryOperator::Multiply,
        ir::BinaryOperator::Divide | ir::BinaryOperator::Modulo => {
            panic!("Unexpected binary operator: {:?}", node)
        }
    }
}

pub fn parse_operand(node: ir::Value) -> Operand {
    match node {
        ir::Value::Constant(val) => Operand::Imm(val),
        ir::Value::Var(identifier) => Operand::PseudoRegister(identifier),
    }
}

pub fn parse_instructions(node: ir::Instruction) -> Vec<Instruction> {
    match node {
        ir::Instruction::Return(val) => {
            let src = parse_operand(val);
            let dst = Operand::Register(Reg::AX);
            vec![Instruction::Mov { src: src, dst: dst }, Instruction::Ret]
        }
        ir::Instruction::Unary { op, src, dst } => {
            let op = parse_unary_operator(op);
            let src = parse_operand(src);
            let dst = parse_operand(dst);
            vec![
                Instruction::Mov {
                    src: src,
                    dst: dst.clone(),
                },
                Instruction::Unary { op: op, dst: dst },
            ]
        }
        ir::Instruction::Binary {
            op,
            left,
            right,
            dst,
        } => {
            let left = parse_operand(left);
            let right = parse_operand(right);
            let dst = parse_operand(dst);
            match op {
                ir::BinaryOperator::Add
                | ir::BinaryOperator::Subtract
                | ir::BinaryOperator::Multiply => {
                    let op = parse_binary_operator(op);
                    vec![
                        Instruction::Mov {
                            src: left,
                            dst: dst.clone(),
                        },
                        Instruction::Binary {
                            op,
                            src: right,
                            dst,
                        },
                    ]
                }
                ir::BinaryOperator::Divide => {
                    vec![
                        Instruction::Mov {
                            src: left,
                            dst: Operand::Register(Reg::AX),
                        },
                        Instruction::Cdq,
                        Instruction::Idiv(right),
                        Instruction::Mov {
                            src: Operand::Register(Reg::AX),
                            dst,
                        },
                    ]
                }
                _ => todo!(),
            }
        }
    }
}

pub fn parse_function_definition(node: ir::FunctionDefinition) -> FunctionDefinition {
    match node {
        ir::FunctionDefinition::Function { identifier, body } => {
            let mut all_asm_instructions = Vec::new();

            for ir_instruction in body.into_iter() {
                let mut asm_instructions = parse_instructions(ir_instruction);
                all_asm_instructions.append(&mut asm_instructions);
            }

            FunctionDefinition::Function {
                name: identifier,
                instructions: all_asm_instructions,
            }
        }
    }
}

pub fn parse_program_definition(node: ir::ProgramDefinition) -> ProgramDefinition {
    match node {
        ir::ProgramDefinition::Program(ir_func_defn) => {
            let asm_function_definition = parse_function_definition(ir_func_defn);
            ProgramDefinition::Program(asm_function_definition)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_ir_constant_to_asm_immediate() {
        let value = 2;
        let ir_ast_node = ir::Value::Constant(value);
        let expected_asm_ast_node = Operand::Imm(value);
        let asm_ast_node = parse_operand(ir_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_ir_var_to_asm_pseudo_register() {
        let identifier = "tmp0";
        let ir_ast_node = ir::Value::Var(identifier.to_string());
        let expected_asm_ast_node = Operand::PseudoRegister(identifier.to_string());
        let asm_ast_node = parse_operand(ir_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_ir_bitwise_complement_operator_to_asm_unary_operator() {
        let ir_ast_node = ir::UnaryOperator::BitwiseComplement;
        let expected_asm_ast_node = UnaryOperator::Not;
        let asm_ast_node = parse_unary_operator(ir_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_ir_negation_operator_to_asm_unary_operator() {
        let ir_ast_node = ir::UnaryOperator::Negation;
        let expected_asm_ast_node = UnaryOperator::Neg;
        let asm_ast_node = parse_unary_operator(ir_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_ir_addition_operator_to_asm_binary_operator() {
        let ir_ast_node = ir::BinaryOperator::Add;
        let expected_asm_ast_node = BinaryOperator::Add;
        let asm_ast_node = parse_binary_operator(ir_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_ir_subtraction_operator_to_asm_binary_operator() {
        let ir_ast_node = ir::BinaryOperator::Subtract;
        let expected_asm_ast_node = BinaryOperator::Subtract;
        let asm_ast_node = parse_binary_operator(ir_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_ir_multiplication_operator_to_asm_binary_operator() {
        let ir_ast_node = ir::BinaryOperator::Multiply;
        let expected_asm_ast_node = BinaryOperator::Multiply;
        let asm_ast_node = parse_binary_operator(ir_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    #[should_panic(expected = "Unexpected binary operator: Divide")]
    fn panic_if_ir_division_operator_given_to_parse_binary_operator() {
        parse_binary_operator(ir::BinaryOperator::Divide);
    }

    #[test]
    #[should_panic(expected = "Unexpected binary operator: Modulo")]
    fn panic_if_ir_modulo_operator_given_to_parse_binary_operator() {
        parse_binary_operator(ir::BinaryOperator::Modulo);
    }

    #[test]
    fn parse_ir_return_instruction_to_asm_instructions() {
        let value = 2;
        let ir_constant_ast_node = ir::Value::Constant(value);
        let ir_return_instruction_ast_node = ir::Instruction::Return(ir_constant_ast_node);
        let expected_asm_ast_instruction_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register(Reg::AX),
            },
            Instruction::Ret,
        ];
        let asm_ast_instruction_nodes = parse_instructions(ir_return_instruction_ast_node);
        assert_eq!(
            asm_ast_instruction_nodes,
            expected_asm_ast_instruction_nodes
        );
    }

    #[test]
    fn parse_ir_unary_operator_instruction_to_asm_instruction() {
        let value = 2;
        let tmp_var_identifier = "tmp0";
        let ir_instruction_ast_node = ir::Instruction::Unary {
            op: ir::UnaryOperator::Negation,
            src: ir::Value::Constant(value),
            dst: ir::Value::Var(tmp_var_identifier.to_string()),
        };
        let expected_asm_instruction_ast_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::PseudoRegister(tmp_var_identifier.to_string()),
            },
            Instruction::Unary {
                op: UnaryOperator::Neg,
                dst: Operand::PseudoRegister(tmp_var_identifier.to_string()),
            },
        ];
        let asm_instruction_ast_nodes = parse_instructions(ir_instruction_ast_node);
        assert_eq!(
            asm_instruction_ast_nodes,
            expected_asm_instruction_ast_nodes
        );
    }

    #[test]
    fn parse_ir_addition_binary_operator_instruction_to_asm_instructions() {
        let left = 1;
        let right = 2;
        let tmp_var_identifier = "tmp0";
        let ir_instruction_ast_node = ir::Instruction::Binary {
            op: ir::BinaryOperator::Add,
            left: ir::Value::Constant(left),
            right: ir::Value::Constant(right),
            dst: ir::Value::Var(tmp_var_identifier.to_string()),
        };
        let expected_asm_ast_instruction_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(left),
                dst: Operand::PseudoRegister(tmp_var_identifier.to_string()),
            },
            Instruction::Binary {
                op: BinaryOperator::Add,
                src: Operand::Imm(right),
                dst: Operand::PseudoRegister(tmp_var_identifier.to_string()),
            },
        ];
        let asm_instruction_ast_nodes = parse_instructions(ir_instruction_ast_node);
        assert_eq!(
            asm_instruction_ast_nodes,
            expected_asm_ast_instruction_nodes
        );
    }

    #[test]
    fn parse_ir_division_binary_operator_instruction_to_asm_instructions() {
        let dividend = 9; // value being divided
        let divisor = 2; // value to divide by
        let tmp_var_identifier = "tmp0";
        let ir_instruction_ast_node = ir::Instruction::Binary {
            op: ir::BinaryOperator::Divide,
            left: ir::Value::Constant(dividend),
            right: ir::Value::Constant(divisor),
            dst: ir::Value::Var(tmp_var_identifier.to_string()),
        };
        let expected_asm_ast_instruction_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(dividend),
                dst: Operand::Register(Reg::AX),
            },
            Instruction::Cdq,
            Instruction::Idiv(Operand::Imm(divisor)),
            Instruction::Mov {
                src: Operand::Register(Reg::AX),
                dst: Operand::PseudoRegister(tmp_var_identifier.to_string()),
            },
        ];
        let asm_instruction_ast_nodes = parse_instructions(ir_instruction_ast_node);
        assert_eq!(
            asm_instruction_ast_nodes,
            expected_asm_ast_instruction_nodes
        );
    }

    #[test]
    fn parse_ir_function_defn_to_asm_function_defn() {
        let value = 2;
        let identifier = "main";
        let ir_constant_ast_node = ir::Value::Constant(value);
        let ir_return_instruction_ast_nodes = vec![ir::Instruction::Return(ir_constant_ast_node)];
        let ir_function_defn_ast_node = ir::FunctionDefinition::Function {
            identifier: identifier.to_string(),
            body: ir_return_instruction_ast_nodes,
        };
        let expected_asm_instructions = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register(Reg::AX),
            },
            Instruction::Ret,
        ];
        let expected_asm_ast_node = FunctionDefinition::Function {
            name: identifier.to_string(),
            instructions: expected_asm_instructions,
        };
        let asm_ast_node = parse_function_definition(ir_function_defn_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_ir_program_definition_to_asm_program_defn() {
        let value = 2;
        let identifier = "main";
        let ir_constant_ast_node = ir::Value::Constant(value);
        let ir_return_instruction_ast_nodes = vec![ir::Instruction::Return(ir_constant_ast_node)];
        let ir_function_defn_ast_node = ir::FunctionDefinition::Function {
            identifier: identifier.to_string(),
            body: ir_return_instruction_ast_nodes,
        };
        let ir_program_defn_ast_node = ir::ProgramDefinition::Program(ir_function_defn_ast_node);
        let asm_instructions = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register(Reg::AX),
            },
            Instruction::Ret,
        ];
        let asm_function_defn_ast_node = FunctionDefinition::Function {
            name: identifier.to_string(),
            instructions: asm_instructions,
        };
        let expected_asm_ast_node = ProgramDefinition::Program(asm_function_defn_ast_node);
        let asm_ast_node = parse_program_definition(ir_program_defn_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }
}
