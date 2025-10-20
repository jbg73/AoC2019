pub mod cpu;

pub enum ProgramName {
    SimpleIntCode,
    SearchNounAndVerb,
}

#[derive(PartialEq)]
pub enum ProgramState {
    Idle,
    Running,
    Finished,
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: i32,
    pub args: Vec<i32>,
}
