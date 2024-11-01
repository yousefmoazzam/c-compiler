use crate::parse::asm::Operand;

pub fn emit_operand(node: Operand) -> String {
    match node {
        Operand::Imm(val) => format!("${}", val),
        Operand::Register => "%eax".to_string(),
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
}
