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
    pub opcode: i32,
    pub args_mode: Vec<ParameterMode>,
}

#[derive(Debug)]
pub struct Instruction {
    pub info: InstructionInfo,
    pub args: Vec<i32>,
}
