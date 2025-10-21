pub mod cpu;

#[derive(PartialEq)]
pub enum ProgramState {
    Idle,
    Running,
    Finished,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
pub struct InstructionInfo {
    pub opcode: Opcode,
    pub args_mode: Vec<ParameterMode>,
}

#[derive(Debug)]
pub struct Instruction {
    pub info: InstructionInfo,
    pub args: Vec<i32>,
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Add,
    Mul,
    Inp,
    Out,
    Hlt,
    Nop,
}

impl From<i32> for Opcode {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Inp,
            4 => Self::Out,
            99 => Self::Hlt,
            _ => Self::Nop,
        }
    }
}
