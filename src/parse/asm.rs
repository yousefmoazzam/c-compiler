use crate::parse::ir;

#[derive(Debug, PartialEq, Clone)]
pub enum Reg {
    AX,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Neg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Imm(u8),
    Register(Reg),
    PseudoRegister(crate::parse::Identifier),
    Stack(i8),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
    Unary { op: UnaryOperator, dst: Operand },
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

pub fn parse_unary_operator(node: ir::UnaryOperator) -> UnaryOperator {
    match node {
        ir::UnaryOperator::BitwiseComplement => UnaryOperator::Not,
        ir::UnaryOperator::Negation => UnaryOperator::Neg,
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
        let ir_constant_ast_node = ir::Value::Constant(value);
        let ir_tmp_var_ast_node = ir::Value::Var(tmp_var_identifier.to_string());
        let ir_instruction_ast_node = ir::Instruction::Unary {
            op: ir::UnaryOperator::Negation,
            src: ir_constant_ast_node,
            dst: ir_tmp_var_ast_node,
        };
        let asm_instructions_same_dst = Operand::PseudoRegister(tmp_var_identifier.to_string());
        let expected_asm_instruction_ast_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: asm_instructions_same_dst.clone(),
            },
            Instruction::Unary {
                op: UnaryOperator::Neg,
                dst: asm_instructions_same_dst,
            },
        ];
        let asm_instruction_ast_nodes = parse_instructions(ir_instruction_ast_node);
        assert_eq!(
            asm_instruction_ast_nodes,
            expected_asm_instruction_ast_nodes
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

mod second_pass {
    use std::collections::HashMap;

    use super::*;

    /// All temporary variables put onto the stack are assumed to be 4-byte integers
    const TMP_VAR_BYTE_LEN: usize = 4;

    pub fn parse_operand(node: Operand, map: &mut HashMap<String, i8>, offset: &mut i8) -> Operand {
        match node {
            Operand::PseudoRegister(identifier) => match map.get(&identifier) {
                Some(value) => Operand::Stack(*value),
                None => {
                    *offset -= TMP_VAR_BYTE_LEN as i8;
                    (*map).insert(identifier.to_string(), *offset);
                    Operand::Stack(*offset)
                }
            },
            _ => node,
        }
    }

    pub fn parse_instructions(nodes: Vec<Instruction>) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let mut offset = 0;
        let mut map: HashMap<String, i8> = HashMap::new();

        for instruction in nodes.into_iter() {
            match instruction {
                Instruction::Mov { src, dst } => {
                    let src = parse_operand(src, &mut map, &mut offset);
                    let dst = parse_operand(dst, &mut map, &mut offset);
                    instructions.push(Instruction::Mov { src, dst });
                }
                Instruction::Unary { op, dst } => {
                    let dst = parse_operand(dst, &mut map, &mut offset);
                    instructions.push(Instruction::Unary { op, dst });
                }
                _ => todo!(),
            }
        }

        instructions
    }

    pub fn parse_function_definition(node: FunctionDefinition) -> FunctionDefinition {
        match node {
            FunctionDefinition::Function { name, instructions } => FunctionDefinition::Function {
                name,
                instructions: parse_instructions(instructions),
            },
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn convert_pseudo_register_to_stack_address_and_update_hash_table_and_offset() {
            let mut offset = 0;
            let mut map: HashMap<String, i8> = HashMap::new();
            let identifier = "tmp0";
            let input_asm_ast_node = Operand::PseudoRegister(identifier.to_string());
            let expected_output_asm_ast_node = Operand::Stack(-(TMP_VAR_BYTE_LEN as i8));
            let transformed_asm_ast_node = parse_operand(input_asm_ast_node, &mut map, &mut offset);
            assert_eq!(-(TMP_VAR_BYTE_LEN as i8), offset);
            assert_eq!(
                true,
                map.get(identifier)
                    .is_some_and(|val| *val == -(TMP_VAR_BYTE_LEN as i8))
            );
            assert_eq!(expected_output_asm_ast_node, transformed_asm_ast_node);
        }

        #[test]
        fn non_pseudo_register_operand_is_left_unchanged() {
            let mut offset = 0;
            let mut map: HashMap<String, i8> = HashMap::new();
            let value = 2;
            let input_asm_ast_node = Operand::Imm(value);
            let output_asm_ast_node =
                parse_operand(input_asm_ast_node.clone(), &mut map, &mut offset);
            assert_eq!(0, offset);
            assert_eq!(0, map.len());
            assert_eq!(input_asm_ast_node, output_asm_ast_node);
        }

        #[test]
        fn pseudo_registers_with_same_identifier_get_same_stack_address() {
            let value = 2;
            let tmp_var_identifier = "tmp0";
            let asm_instructions_same_dst = Operand::PseudoRegister(tmp_var_identifier.to_string());
            let input_asm_instruction_ast_nodes = vec![
                Instruction::Mov {
                    src: Operand::Imm(value),
                    dst: asm_instructions_same_dst.clone(),
                },
                Instruction::Unary {
                    op: UnaryOperator::Neg,
                    dst: asm_instructions_same_dst,
                },
            ];
            let expected_asm_instructions_same_stack_addr_dst =
                Operand::Stack(-(TMP_VAR_BYTE_LEN as i8));
            let expected_asm_instruction_ast_nodes = vec![
                Instruction::Mov {
                    src: Operand::Imm(value),
                    dst: expected_asm_instructions_same_stack_addr_dst.clone(),
                },
                Instruction::Unary {
                    op: UnaryOperator::Neg,
                    dst: expected_asm_instructions_same_stack_addr_dst,
                },
            ];
            let output_asm_instruction_ast_nodes =
                parse_instructions(input_asm_instruction_ast_nodes);
            assert_eq!(
                expected_asm_instruction_ast_nodes,
                output_asm_instruction_ast_nodes
            );
        }

        #[test]
        fn function_defn_name_left_unchanged() {
            let value = 2;
            let tmp_var_identifier = "tmp0";
            let function_name_identifier = "main";

            let asm_instructions_same_dst = Operand::PseudoRegister(tmp_var_identifier.to_string());
            let asm_instruction_ast_nodes = vec![
                Instruction::Mov {
                    src: Operand::Imm(value),
                    dst: asm_instructions_same_dst.clone(),
                },
                Instruction::Unary {
                    op: UnaryOperator::Neg,
                    dst: asm_instructions_same_dst,
                },
            ];
            let input_function_defn_asm_ast_node = FunctionDefinition::Function {
                name: function_name_identifier.to_string(),
                instructions: asm_instruction_ast_nodes,
            };

            let expected_asm_instructions_same_stack_addr_dst =
                Operand::Stack(-(TMP_VAR_BYTE_LEN as i8));
            let expected_asm_instruction_ast_nodes = vec![
                Instruction::Mov {
                    src: Operand::Imm(value),
                    dst: expected_asm_instructions_same_stack_addr_dst.clone(),
                },
                Instruction::Unary {
                    op: UnaryOperator::Neg,
                    dst: expected_asm_instructions_same_stack_addr_dst,
                },
            ];
            let expected_output_function_defn_asm_ast_node = FunctionDefinition::Function {
                name: function_name_identifier.to_string(),
                instructions: expected_asm_instruction_ast_nodes,
            };

            let output_function_defn_asm_ast_node =
                parse_function_definition(input_function_defn_asm_ast_node);
            assert_eq!(
                expected_output_function_defn_asm_ast_node,
                output_function_defn_asm_ast_node
            );
        }
    }
}
