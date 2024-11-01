use crate::parse::asm::{FunctionDefinition, Instruction, Operand};

pub fn emit_operand(node: Operand) -> String {
    match node {
        Operand::Imm(val) => format!("${}", val),
        Operand::Register => "%eax".to_string(),
    }
}

pub fn emit_instruction(node: Instruction) -> String {
    match node {
        Instruction::Mov { src, dst } => {
            let src_string = emit_operand(src);
            let dst_string = emit_operand(dst);
            format!("    movl {}, {}", src_string, dst_string)
        }
        Instruction::Ret => "    ret".to_string(),
    }
}

pub fn emit_function_definition(node: FunctionDefinition) -> Vec<String> {
    match node {
        FunctionDefinition::Function { name, instructions } => {
            let mut lines = vec![format!("    .globl {}", name), format!("{}", name)];
            for instruction in instructions {
                let instruction_string = emit_instruction(instruction);
                lines.push(instruction_string);
            }
            lines
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_imm_operand() {
        let value = 2;
        let ast_node = Operand::Imm(value);
        let asm_code = emit_operand(ast_node);
        let expected_asm_code = "$2";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_register_operand() {
        let ast_node = Operand::Register;
        let asm_code = emit_operand(ast_node);
        let expected_asm_code = "%eax";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_mov_instruction() {
        let value = 2;
        let ast_node = Instruction::Mov {
            src: Operand::Imm(value),
            dst: Operand::Register,
        };
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = "    movl $2, %eax";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_ret_instruction() {
        let ast_node = Instruction::Ret;
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = "    ret";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_function_definition_returns_correct_vector_of_strings() {
        let value = 2;
        let identifier = "main";
        let instructions = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register,
            },
            Instruction::Ret,
        ];
        let ast_node = FunctionDefinition::Function {
            name: identifier.to_string(),
            instructions,
        };
        let asm_code = emit_function_definition(ast_node);
        let expected_asm_code = vec![
            format!("    .globl {}", identifier.to_string()),
            format!("{}", identifier.to_string()),
            format!("    movl ${}, %eax", value),
            "    ret".to_string(),
        ];
        assert_eq!(asm_code, expected_asm_code);
    }
}
