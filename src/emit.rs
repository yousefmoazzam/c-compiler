use std::path::Path;

use crate::parse::asm::{
    BinaryOperator, FunctionDefinition, Instruction, Operand, ProgramDefinition, Reg, UnaryOperator,
};

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
            Reg::DX => "%edx".to_string(),
            Reg::R10D => "%r10d".to_string(),
            Reg::R11D => "%r11d".to_string(),
        },
        Operand::Stack(offset) => format!("{}(%rbp)", offset),
        Operand::PseudoRegister(_) => {
            panic!("Pseudo-register operand is invalid at code emission stage")
        }
    }
}

pub fn emit_unary_operator(node: UnaryOperator) -> String {
    match node {
        UnaryOperator::Neg => "negl".to_string(),
        UnaryOperator::Not => "notl".to_string(),
    }
}

fn emit_binary_operator(node: BinaryOperator) -> String {
    match node {
        BinaryOperator::Add => "addl".to_string(),
        BinaryOperator::Subtract => "subl".to_string(),
        BinaryOperator::Multiply => "imull".to_string(),
    }
}

pub fn emit_instruction(node: Instruction) -> Vec<String> {
    let mut lines = Vec::new();

    match node {
        Instruction::Mov { src, dst } => {
            let src_string = emit_operand(src);
            let dst_string = emit_operand(dst);
            lines.push(format!("    movl {}, {}", src_string, dst_string));
        }
        Instruction::Ret => {
            lines.append(&mut vec![
                "    movq %rbp, %rsp".to_string(),
                "    popq %rbp".to_string(),
                "    ret".to_string(),
            ]);
        }
        Instruction::AllocateStack(offset) => lines.push(format!("    subq ${}, %rsp", offset)),
        Instruction::Unary { op, dst } => {
            let op_string = emit_unary_operator(op);
            let dst_string = emit_operand(dst);
            lines.push(format!("    {} {}", op_string, dst_string));
        }
        Instruction::Cdq => lines.push("    cdq".to_string()),
        Instruction::Idiv(operand) => {
            let operand = emit_operand(operand);
            lines.push(format!("    idivl {}", operand));
        }
        Instruction::Binary { op, src, dst } => {
            let op = emit_binary_operator(op);
            let src = emit_operand(src);
            let dst = emit_operand(dst);
            lines.push(format!("    {} {}, {}", op, src, dst));
        }
    }

    lines
}

pub fn emit_function_definition(node: FunctionDefinition) -> Vec<String> {
    match node {
        FunctionDefinition::Function { name, instructions } => {
            let mut lines = vec![
                format!("    .globl {}", name),
                format!("{}:", name),
                "    pushq %rbp".to_string(),
                "    movq %rsp, %rbp".to_string(),
            ];
            for instruction in instructions {
                let mut instruction_strings = emit_instruction(instruction);
                lines.append(&mut instruction_strings);
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
    fn emit_register_dx_operand() {
        let ast_node = Operand::Register(Reg::DX);
        let asm_code = emit_operand(ast_node);
        let expected_asm_code = "%edx";
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
    fn emit_register_r11d_operand() {
        let ast_node = Operand::Register(Reg::R11D);
        let asm_code = emit_operand(ast_node);
        let expected_asm_code = "%r11d";
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
    fn emit_neg_unary_operator() {
        let ast_node = UnaryOperator::Neg;
        let asm_code = emit_unary_operator(ast_node);
        let expected_asm_code = "negl";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_not_unary_operator() {
        let ast_node = UnaryOperator::Not;
        let asm_code = emit_unary_operator(ast_node);
        let expected_asm_code = "notl";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_add_binary_operator() {
        let ast_node = BinaryOperator::Add;
        let asm_code = emit_binary_operator(ast_node);
        let expected_asm_code = "addl";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_subtract_binary_operator() {
        let ast_node = BinaryOperator::Subtract;
        let asm_code = emit_binary_operator(ast_node);
        let expected_asm_code = "subl";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_multiply_binary_operator() {
        let ast_node = BinaryOperator::Multiply;
        let asm_code = emit_binary_operator(ast_node);
        let expected_asm_code = "imull";
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_mov_instruction() {
        let value = 2;
        let ast_node = Instruction::Mov {
            src: Operand::Imm(value),
            dst: Operand::Register(Reg::AX),
        };
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = vec!["    movl $2, %eax"];
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_ret_instruction() {
        let ast_node = Instruction::Ret;
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = vec![
            "    movq %rbp, %rsp".to_string(),
            "    popq %rbp".to_string(),
            "    ret".to_string(),
        ];
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_allocate_stack_instruction() {
        let offset = 8;
        let ast_node = Instruction::AllocateStack(offset);
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = vec![format!("    subq ${}, %rsp", offset)];
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_unary_instruction() {
        let value = 2;
        let ast_node = Instruction::Unary {
            op: UnaryOperator::Neg,
            dst: Operand::Imm(value),
        };
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = vec![format!("    negl ${}", value)];
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_cdq_instruction() {
        let asm_code = emit_instruction(Instruction::Cdq);
        let expected_asm_code = vec!["    cdq"];
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_idiv_instruction() {
        let ast_node = Instruction::Idiv(Operand::Register(Reg::R10D));
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = vec!["    idivl %r10d"];
        assert_eq!(asm_code, expected_asm_code);
    }

    #[test]
    fn emit_binary_instruction() {
        let src = 2;
        let ast_node = Instruction::Binary {
            op: BinaryOperator::Add,
            src: Operand::Imm(src),
            dst: Operand::Register(Reg::AX),
        };
        let asm_code = emit_instruction(ast_node);
        let expected_asm_code = vec![format!("    addl ${}, %eax", src)];
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
            "    pushq %rbp".to_string(),
            "    movq %rsp, %rbp".to_string(),
            format!("    movl ${}, %eax", value),
            "    movq %rbp, %rsp".to_string(),
            "    popq %rbp".to_string(),
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
            "    pushq %rbp".to_string(),
            "    movq %rsp, %rbp".to_string(),
            format!("    movl ${}, %eax", value),
            "    movq %rbp, %rsp".to_string(),
            "    popq %rbp".to_string(),
            "    ret".to_string(),
        ];
        assert_eq!(asm_code, expected_asm_code);
    }
}
