use crate::parse::asm::{FunctionDefinition, Instruction, Operand, ProgramDefinition, Reg};

pub fn parse_program_definition(node: ProgramDefinition, stack_offset: i8) -> ProgramDefinition {
    match node {
        ProgramDefinition::Program(func_defn) => {
            ProgramDefinition::Program(parse_function_definition(func_defn, stack_offset))
        }
    }
}

pub fn parse_function_definition(node: FunctionDefinition, stack_offset: i8) -> FunctionDefinition {
    match node {
        FunctionDefinition::Function {
            name,
            mut instructions,
        } => {
            // NOTE: Inserting at the front of a vector is the worst case scenario (all elements
            // need to be shifted), so might be worth rethinking this at some point.
            instructions.insert(0, Instruction::AllocateStack(-(stack_offset) as u8));
            FunctionDefinition::Function {
                name,
                instructions: parse_instructions(instructions),
            }
        }
    }
}

pub fn parse_instructions(nodes: Vec<Instruction>) -> Vec<Instruction> {
    let mut transformed_instructions = Vec::new();

    for node in nodes.into_iter() {
        match node {
            Instruction::Mov {
                src: Operand::Stack(src_offset),
                dst: Operand::Stack(dst_offset),
            } => {
                let mut intermediate_register_instructions = vec![
                    Instruction::Mov {
                        src: Operand::Stack(src_offset),
                        dst: Operand::Register(Reg::R10D),
                    },
                    Instruction::Mov {
                        src: Operand::Register(Reg::R10D),
                        dst: Operand::Stack(dst_offset),
                    },
                ];
                transformed_instructions.append(&mut intermediate_register_instructions);
            }
            _ => transformed_instructions.push(node),
        }
    }

    transformed_instructions
}

#[cfg(test)]
mod tests {
    use crate::parse::asm::{Operand, UnaryOperator, TMP_VAR_BYTE_LEN};

    use super::*;

    #[test]
    fn insert_stack_frame_allocate_instruction_at_start_of_function_defn_instructions() {
        let value = 2;
        let function_name_identifier = "main";
        let stack_offset = -(TMP_VAR_BYTE_LEN as i8);

        let asm_instructions_same_stack_addr_dst = Operand::Stack(stack_offset);
        let asm_instruction_ast_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: asm_instructions_same_stack_addr_dst.clone(),
            },
            Instruction::Unary {
                op: UnaryOperator::Neg,
                dst: asm_instructions_same_stack_addr_dst,
            },
        ];
        let input_asm_function_defn_ast_node = FunctionDefinition::Function {
            name: function_name_identifier.to_string(),
            instructions: asm_instruction_ast_nodes,
        };

        let expected_asm_instructions_same_stack_addr_dst = Operand::Stack(stack_offset);
        let expected_asm_instruction_ast_nodes = vec![
            Instruction::AllocateStack(-(stack_offset) as u8),
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: expected_asm_instructions_same_stack_addr_dst.clone(),
            },
            Instruction::Unary {
                op: UnaryOperator::Neg,
                dst: expected_asm_instructions_same_stack_addr_dst,
            },
        ];
        let expected_output_asm_function_defn_ast_node = FunctionDefinition::Function {
            name: function_name_identifier.to_string(),
            instructions: expected_asm_instruction_ast_nodes,
        };
        let output_asm_function_defn_ast_nodes =
            parse_function_definition(input_asm_function_defn_ast_node, stack_offset);

        assert_eq!(
            expected_output_asm_function_defn_ast_node,
            output_asm_function_defn_ast_nodes
        );
    }

    #[test]
    fn convert_mov_instructions_with_src_dst_stack_addrs_to_two_mov_instructions() {
        // Mov(Operand::Imm(2), Operand::Stack(-4))
        // Unary(UnaryOperator::Not, Operand::Stack(-4))
        //
        // Mov(Operand::Stack(-4), Operand::Register(Reg::R10D))
        // Mov(Operand::Register(Reg::R10D), Operand::Stack(-8))
        //
        // Unary(UnaryOperator::Neg, Operand::Stack(-8))
        // Mov(Operand::Stack(-8), Operand::Register(Reg::AX))
        // Ret
        let value = 2;
        let input_asm_instruction_ast_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Stack(-4),
            },
            Instruction::Unary {
                op: UnaryOperator::Not,
                dst: Operand::Stack(-4),
            },
            Instruction::Mov {
                src: Operand::Stack(-4),
                dst: Operand::Stack(-8),
            },
            Instruction::Unary {
                op: UnaryOperator::Neg,
                dst: Operand::Stack(-8),
            },
            Instruction::Mov {
                src: Operand::Stack(-8),
                dst: Operand::Register(Reg::AX),
            },
            Instruction::Ret,
        ];

        let expected_asm_instruction_ast_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Stack(-4),
            },
            Instruction::Unary {
                op: UnaryOperator::Not,
                dst: Operand::Stack(-4),
            },
            Instruction::Mov {
                src: Operand::Stack(-4),
                dst: Operand::Register(Reg::R10D),
            },
            Instruction::Mov {
                src: Operand::Register(Reg::R10D),
                dst: Operand::Stack(-8),
            },
            Instruction::Unary {
                op: UnaryOperator::Neg,
                dst: Operand::Stack(-8),
            },
            Instruction::Mov {
                src: Operand::Stack(-8),
                dst: Operand::Register(Reg::AX),
            },
            Instruction::Ret,
        ];

        let output_asm_ast_instruction_ast_nodes =
            parse_instructions(input_asm_instruction_ast_nodes);
        assert_eq!(
            expected_asm_instruction_ast_nodes,
            output_asm_ast_instruction_ast_nodes
        );
    }
}
