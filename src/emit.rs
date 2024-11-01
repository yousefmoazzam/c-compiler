use crate::parse::asm::{Instruction, Operand};

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
        _ => todo!(),
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
}
