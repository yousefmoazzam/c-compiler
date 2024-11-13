use std::path::Path;

use crate::parse::asm::{FunctionDefinition, Instruction, Operand, ProgramDefinition, Reg};

pub fn emit(output: &Path, node: ProgramDefinition) -> std::io::Result<()> {
    let lines = emit_program_definition(node);
    let joined_lines = lines.join("\n");
    std::fs::write(output, joined_lines).expect("Unable to write assembly code to file");
    Ok(())
}

pub fn emit_operand(node: Operand) -> String {
    match node {
        Operand::Imm(val) => format!("${}", val),
        Operand::Register(reg) => match reg {
            Reg::AX => "%eax".to_string(),
            Reg::R10D => "%r10d".to_string(),
        },
        Operand::Stack(offset) => format!("{}(%rbp)", offset),
        Operand::PseudoRegister(_) => {
            panic!("Pseudo-register operand is invalid at code emission stage")
        }
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
        _ => todo!(),
    }
}

pub fn emit_function_definition(node: FunctionDefinition) -> Vec<String> {
    match node {
        FunctionDefinition::Function { name, instructions } => {
            let mut lines = vec![format!("    .globl {}", name), format!("{}:", name)];
            for instruction in instructions {
                let instruction_string = emit_instruction(instruction);
                lines.push(instruction_string);
            }
            lines
        }
    }
}

pub fn emit_program_definition(node: ProgramDefinition) -> Vec<String> {
    match node {
        ProgramDefinition::Program(func_defn) => emit_function_definition(func_defn),
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
    fn emit_register_ax_operand() {
        let ast_node = Operand::Register(Reg::AX);
        let asm_code = emit_operand(ast_node);
        let expected_asm_code = "%eax";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_register_r10d_operand() {
        let ast_node = Operand::Register(Reg::R10D);
        let asm_code = emit_operand(ast_node);
        let expected_asm_code = "%r10d";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_stack_addr_operand() {
        let offset = -4;
        let ast_node = Operand::Stack(offset);
        let asm_code = emit_operand(ast_node);
        let expected_asm_code = format!("{}(%rbp)", offset);
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    #[should_panic(expected = "Pseudo-register operand is invalid at code emission stage")]
    fn panic_if_pseudo_register_operand_encountered() {
        let ast_node = Operand::PseudoRegister("tmp0".to_string());
        emit_operand(ast_node);
    }

    #[test]
    fn emit_mov_instruction() {
        let value = 2;
        let ast_node = Instruction::Mov {
            src: Operand::Imm(value),
            dst: Operand::Register(Reg::AX),
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
                dst: Operand::Register(Reg::AX),
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
            format!("{}:", identifier.to_string()),
            format!("    movl ${}, %eax", value),
            "    ret".to_string(),
        ];
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_program_definition_returns_correct_vector_of_strings() {
        let value = 2;
        let identifier = "main";
        let instructions = vec![
            Instruction::Mov {
                src: Operand::Imm(value),
                dst: Operand::Register(Reg::AX),
            },
            Instruction::Ret,
        ];
        let function_defn = FunctionDefinition::Function {
            name: identifier.to_string(),
            instructions,
        };
        let ast_node = ProgramDefinition::Program(function_defn);
        let asm_code = emit_program_definition(ast_node);
        let expected_asm_code = vec![
            format!("    .globl {}", identifier.to_string()),
            format!("{}:", identifier.to_string()),
            format!("    movl ${}, %eax", value),
            "    ret".to_string(),
        ];
        assert_eq!(asm_code, expected_asm_code);
    }
}
