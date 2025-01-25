use crate::parse::asm::{
    FunctionDefinition, Instruction, Operand, ProgramDefinition, TMP_VAR_BYTE_LEN,
};

use std::collections::HashMap;

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

pub fn parse_instructions(nodes: Vec<Instruction>, stack_offset: &mut i8) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut map: HashMap<String, i8> = HashMap::new();

    for instruction in nodes.into_iter() {
        match instruction {
            Instruction::Mov { src, dst } => {
                let src = parse_operand(src, &mut map, stack_offset);
                let dst = parse_operand(dst, &mut map, stack_offset);
                instructions.push(Instruction::Mov { src, dst });
            }
            Instruction::Unary { op, dst } => {
                let dst = parse_operand(dst, &mut map, stack_offset);
                instructions.push(Instruction::Unary { op, dst });
            }
            Instruction::AllocateStack(_) => {
                panic!("Stack allocation instruction shouldn't be present in second pass")
            }
            Instruction::Ret => instructions.push(instruction),
            Instruction::Binary { op, src, dst } => {
                let src = parse_operand(src, &mut map, stack_offset);
                let dst = parse_operand(dst, &mut map, stack_offset);
                instructions.push(Instruction::Binary { op, src, dst });
            }
            Instruction::Idiv(operand) => {
                let operand = parse_operand(operand, &mut map, stack_offset);
                instructions.push(Instruction::Idiv(operand));
            }
            _ => todo!(),
        }
    }

    instructions
}

pub fn parse_function_definition(
    node: FunctionDefinition,
    stack_offset: &mut i8,
) -> FunctionDefinition {
    match node {
        FunctionDefinition::Function { name, instructions } => FunctionDefinition::Function {
            name,
            instructions: parse_instructions(instructions, stack_offset),
        },
    }
}

pub fn parse_program_definition(node: ProgramDefinition) -> (ProgramDefinition, i8) {
    let mut stack_offset = 0;

    match node {
        ProgramDefinition::Program(func_defn) => {
            let program_defn =
                ProgramDefinition::Program(parse_function_definition(func_defn, &mut stack_offset));
            (program_defn, stack_offset)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::parse::asm::{BinaryOperator, UnaryOperator};

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
        let output_asm_ast_node = parse_operand(input_asm_ast_node.clone(), &mut map, &mut offset);
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
        let mut stack_offset = 0;
        let output_asm_instruction_ast_nodes =
            parse_instructions(input_asm_instruction_ast_nodes, &mut stack_offset);
        assert_eq!(
            expected_asm_instruction_ast_nodes,
            output_asm_instruction_ast_nodes
        );
    }

    #[test]
    fn pseudo_register_in_addition_binary_operator_instruction_transformed_to_stack_address() {
        // The move instruction isn't strictly needed for the purpose of this test. However, the
        // move instruction is the only part that refers to the left operand of the binary
        // operator. Omitting the move instruction would imply that the left operand is omitted as
        // well, but it looks confusing to have a test involving a binary operator application that
        // omits the left operand. So, the move instruction has been left in for the moment.
        let left = 1;
        let right = 2;
        let tmp_var_identifier = "tmp0";
        let input_asm_instruction_ast_nodes = vec![
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
        let expected_asm_instructions_same_stack_addr_dst =
            Operand::Stack(-(TMP_VAR_BYTE_LEN as i8));
        let expected_asm_instruction_ast_nodes = vec![
            Instruction::Mov {
                src: Operand::Imm(left),
                dst: expected_asm_instructions_same_stack_addr_dst.clone(),
            },
            Instruction::Binary {
                op: BinaryOperator::Add,
                src: Operand::Imm(right),
                dst: expected_asm_instructions_same_stack_addr_dst,
            },
        ];
        let mut stack_offset = 0;
        let output_asm_instruction_ast_nodes =
            parse_instructions(input_asm_instruction_ast_nodes, &mut stack_offset);
        assert_eq!(
            expected_asm_instruction_ast_nodes,
            output_asm_instruction_ast_nodes
        );
    }

    #[test]
    fn pseudo_register_in_division_instruction_transformed_to_stack_address() {
        let input_asm_instruction_ast_nodes = vec![Instruction::Idiv(Operand::PseudoRegister(
            "tmp0".to_string(),
        ))];
        let expected_asm_instruction_ast_nodes =
            vec![Instruction::Idiv(Operand::Stack(-(TMP_VAR_BYTE_LEN as i8)))];
        let mut stack_offset = 0;
        let output_asm_instruction_ast_nodes =
            parse_instructions(input_asm_instruction_ast_nodes, &mut stack_offset);
        assert_eq!(
            expected_asm_instruction_ast_nodes,
            output_asm_instruction_ast_nodes
        );
    }

    #[test]
    #[should_panic(expected = "Stack allocation instruction shouldn't be present in second pass")]
    fn panic_if_allocate_stack_instruction_encountered() {
        let mut stack_offset = -4;
        let input_asm_instruction_ast_nodes =
            vec![Instruction::AllocateStack(-(stack_offset) as u8)];
        _ = parse_instructions(input_asm_instruction_ast_nodes, &mut stack_offset)
    }

    #[test]
    fn dont_transform_return_instruction() {
        let mut stack_offset = -4;
        let input_asm_instruction_ast_nodes = vec![Instruction::Ret];
        let output_asm_instruction_ast_nodes =
            parse_instructions(input_asm_instruction_ast_nodes, &mut stack_offset);
        assert_eq!(vec![Instruction::Ret], output_asm_instruction_ast_nodes);
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

        let mut stack_offset = 0;
        let output_function_defn_asm_ast_node =
            parse_function_definition(input_function_defn_asm_ast_node, &mut stack_offset);
        assert_eq!(
            expected_output_function_defn_asm_ast_node,
            output_function_defn_asm_ast_node
        );
    }

    #[test]
    fn program_parsing_returns_final_stack_offset_and_correct_program_defn() {
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
        let function_defn_asm_ast_node = FunctionDefinition::Function {
            name: function_name_identifier.to_string(),
            instructions: asm_instruction_ast_nodes,
        };
        let input_program_defn_ast_node = ProgramDefinition::Program(function_defn_asm_ast_node);

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
        let output_function_defn_asm_ast_node = FunctionDefinition::Function {
            name: function_name_identifier.to_string(),
            instructions: expected_asm_instruction_ast_nodes,
        };
        let expected_program_defn_asm_ast_node =
            ProgramDefinition::Program(output_function_defn_asm_ast_node);
        let expected_stack_offset = -(TMP_VAR_BYTE_LEN as i8);

        let (output_program_defn_ast_node, output_stack_offset) =
            parse_program_definition(input_program_defn_ast_node);
        assert_eq!(
            expected_program_defn_asm_ast_node,
            output_program_defn_ast_node
        );
        assert_eq!(expected_stack_offset, output_stack_offset);
    }
}
