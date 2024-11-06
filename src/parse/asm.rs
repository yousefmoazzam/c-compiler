use crate::parse::c;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm(u8),
    // NOTE: Only need to use a single register (`EAX`) as an instruction operand so far, so
    // there's no need to be able to specify which register is being used. Thus, for now,
    // always assume this variant to mean the `EAX` register.
    Register,
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

pub fn parse_operand(node: c::Expression) -> Operand {
    match node {
        c::Expression::NumericConstant(val) => Operand::Imm(val),
        _ => todo!(),
    }
}

pub fn parse_instructions(node: c::Statement) -> Vec<Instruction> {
    match node {
        c::Statement::Return(exp) => {
            let src = parse_operand(exp);
            let dst = Operand::Register;
            vec![Instruction::Mov { src: src, dst: dst }]
        }
    }
}

pub fn parse_function_definition(node: c::FunctionDefinition) -> FunctionDefinition {
    match node {
        c::FunctionDefinition::Function { name, body } => {
            let instructions = parse_instructions(body);
            FunctionDefinition::Function { name, instructions }
        }
    }
}

pub fn parse_program_definition(node: c::ProgramDefinition) -> ProgramDefinition {
    match node {
        c::ProgramDefinition::Program(c_func_defn) => {
            let asm_function_definition = parse_function_definition(c_func_defn);
            ProgramDefinition::Program(asm_function_definition)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_c_constant_to_asm_immediate() {
        let value = 2;
        let c_ast_node = c::Expression::NumericConstant(value);
        let expected_asm_ast_node = Operand::Imm(value);
        let asm_ast_node = parse_operand(c_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_c_return_to_asm_instructions() {
        let value = 2;
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let c_return_ast_node = c::Statement::Return(c_constant_ast_node);
        let expected_asm_ast_instruction_nodes = vec![Instruction::Mov {
            src: Operand::Imm(value),
            dst: Operand::Register,
        }];
        let asm_ast_instruction_nodes = parse_instructions(c_return_ast_node);
        assert_eq!(
            asm_ast_instruction_nodes,
            expected_asm_ast_instruction_nodes
        );
    }

    #[test]
    fn parse_c_function_defn_to_asm_function_defn() {
        let value = 2;
        let identifier = "main";
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let c_return_ast_node = c::Statement::Return(c_constant_ast_node);
        let c_function_defn_ast_node = c::FunctionDefinition::Function {
            name: identifier.to_string(),
            body: c_return_ast_node,
        };
        let expected_asm_instructions = vec![Instruction::Mov {
            src: Operand::Imm(value),
            dst: Operand::Register,
        }];
        let expected_asm_ast_node = FunctionDefinition::Function {
            name: identifier.to_string(),
            instructions: expected_asm_instructions,
        };
        let asm_ast_node = parse_function_definition(c_function_defn_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }

    #[test]
    fn parse_c_program_definition_to_asm_program_defn() {
        let value = 2;
        let identifier = "main";
        let c_constant_ast_node = c::Expression::NumericConstant(value);
        let c_return_ast_node = c::Statement::Return(c_constant_ast_node);
        let c_function_defn_ast_node = c::FunctionDefinition::Function {
            name: identifier.to_string(),
            body: c_return_ast_node,
        };
        let c_program_defn_ast_node = c::ProgramDefinition::Program(c_function_defn_ast_node);
        let asm_instructions = vec![Instruction::Mov {
            src: Operand::Imm(value),
            dst: Operand::Register,
        }];
        let asm_function_defn_ast_node = FunctionDefinition::Function {
            name: identifier.to_string(),
            instructions: asm_instructions,
        };
        let expected_asm_ast_node = ProgramDefinition::Program(asm_function_defn_ast_node);
        let asm_ast_node = parse_program_definition(c_program_defn_ast_node);
        assert_eq!(asm_ast_node, expected_asm_ast_node);
    }
}
