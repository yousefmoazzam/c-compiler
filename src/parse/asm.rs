pub mod first_pass;
mod second_pass;

/// All temporary variables put onto the stack are assumed to be 4-byte integers
const TMP_VAR_BYTE_LEN: usize = 4;

#[derive(Debug, PartialEq, Clone)]
pub enum Reg {
    AX,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Neg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Imm(u8),
    Register(Reg),
    PseudoRegister(crate::parse::Identifier),
    Stack(i8),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
    Unary { op: UnaryOperator, dst: Operand },
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
