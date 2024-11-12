use crate::parse::asm::{FunctionDefinition, Instruction};

pub fn parse_function_definition(node: FunctionDefinition, stack_offset: i8) -> FunctionDefinition {
    match node {
        FunctionDefinition::Function {
            name,
            mut instructions,
        } => {
            // NOTE: Inserting at the front of a vector is the worst case scenario (all elements
            // need to be shifted), so might be worth rethinking this at some point.
            instructions.insert(0, Instruction::AllocateStack(stack_offset));
            FunctionDefinition::Function { name, instructions }
        }
    }
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
            Instruction::AllocateStack(stack_offset),
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
}
