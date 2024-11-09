use crate::parse::ir;

#[derive(Debug, PartialEq)]
pub enum Reg {
    AX,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm(u8),
    Register(Reg),
    PseudoRegister(crate::parse::Identifier),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
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
        _ => todo!(),
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
