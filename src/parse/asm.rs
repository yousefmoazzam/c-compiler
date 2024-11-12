pub mod first_pass;
mod second_pass;

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
