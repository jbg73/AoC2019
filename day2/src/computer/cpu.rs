use super::{Instruction, InstructionInfo, Opcode, ParameterMode, ProgramState};
use crate::parser::file_parser::IntCodeParser;

pub struct Computer {
    current_address: usize,
    state: ProgramState,
    input: i32,
}

impl Computer {
    // returns the number of "jumps" to do on the "sp"
    fn get_operation_count(opcode: Opcode) -> usize {
        match opcode {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Inp => 1,
            Opcode::Out => 1,
            Opcode::Hlt => 0,
            Opcode::Nop => 0,
            Opcode::Jit => 2,
            Opcode::Jif => 2,
            Opcode::Let => 3,
            Opcode::Eqs => 3,
        }
    }

    fn read_instruction(mut opcode: i32) -> InstructionInfo {
        let opcode_parsed = opcode % 100;

        opcode /= 100;

        let mut param_modes: Vec<ParameterMode> = Vec::new();
        while opcode >= 10 {
            let mode = opcode % 10;
            let mode = match mode {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => ParameterMode::Position,
            };
            param_modes.push(mode);

            opcode /= 10;
        }
        match opcode {
            1 => param_modes.push(ParameterMode::Immediate),
            _ => param_modes.push(ParameterMode::Position),
        }

        while param_modes.len() < 5 {
            param_modes.push(ParameterMode::Position);
        }

        InstructionInfo {
            opcode: opcode_parsed.into(),
            args_mode: param_modes,
        }
    }

    fn add2(val_l: i32, val_r: i32) -> i32 {
        val_l + val_r
    }

    fn multiply2(val_l: i32, val_r: i32) -> i32 {
        val_l * val_r
    }
}

impl Computer {
    pub fn new() -> Self {
        Self {
            current_address: 0,
            state: ProgramState::Idle,
            input: 5, // TODO: read problem and properly handle input. Not Hardcoded
        }
    }

    pub fn reset(&mut self) {
        self.current_address = 0;
        self.state = ProgramState::Idle;
    }

    pub fn resolve_argument(intcode_data: &[i32], value: i32, mode: ParameterMode) -> i32 {
        if mode == ParameterMode::Position {
            return intcode_data[value as usize];
        }
        value
    }

    fn fetch_instruction(&mut self, intcode_data: &[i32]) -> Instruction {
        let next_intcode = intcode_data[self.current_address];
        let instruction = Computer::read_instruction(next_intcode);

        self.current_address += 1;

        let n_args = Computer::get_operation_count(instruction.opcode);

        let start_idx = self.current_address;
        let end_idx = start_idx + n_args;
        let fetched_args: Vec<i32> = intcode_data[start_idx..end_idx].to_vec();
        self.current_address += n_args;

        Instruction {
            info: instruction,
            args: fetched_args,
        }
    }

    fn execute_intcode(&mut self, mut intcode_data: Vec<i32>) -> Vec<i32> {
        self.state = ProgramState::Running;

        while (self.current_address < intcode_data.len()) && (self.state == ProgramState::Running) {
            let instruction = self.fetch_instruction(&intcode_data);

            match instruction.info.opcode {
                Opcode::Add => {
                    let operand1 = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[0],
                        instruction.info.args_mode[0],
                    );
                    let operand2 = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[1],
                        instruction.info.args_mode[1],
                    );
                    let dst = instruction.args[2];

                    let result = Computer::add2(operand1, operand2);
                    intcode_data[dst as usize] = result;
                }
                Opcode::Mul => {
                    let operand1 = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[0],
                        instruction.info.args_mode[0],
                    );
                    let operand2 = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[1],
                        instruction.info.args_mode[1],
                    );
                    let dst = instruction.args[2];

                    let result = Computer::multiply2(operand1, operand2);
                    intcode_data[dst as usize] = result;
                }
                Opcode::Inp => {
                    let dst = instruction.args[0];
                    intcode_data[dst as usize] = self.input;
                }
                Opcode::Out => {
                    // let src = instruction.args[0];
                    let output = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[0],
                        instruction.info.args_mode[0],
                    );
                    println!("{}", output);
                }
                Opcode::Jit => {
                    let value = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[0],
                        instruction.info.args_mode[0],
                    );
                    if value > 0 {
                        let dst = Computer::resolve_argument(
                            &intcode_data,
                            instruction.args[1],
                            instruction.info.args_mode[1],
                        );
                        self.current_address = dst as usize;
                    }
                }
                Opcode::Jif => {
                    let value = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[0],
                        instruction.info.args_mode[0],
                    );
                    if value == 0 {
                        let dst = Computer::resolve_argument(
                            &intcode_data,
                            instruction.args[1],
                            instruction.info.args_mode[1],
                        );
                        self.current_address = dst as usize;
                    }
                }
                Opcode::Let => {
                    let lhs = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[0],
                        instruction.info.args_mode[0],
                    );
                    let rhs = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[1],
                        instruction.info.args_mode[1],
                    );
                    let dst = instruction.args[2];

                    if lhs < rhs {
                        intcode_data[dst as usize] = 1;
                    } else {
                        intcode_data[dst as usize] = 0;
                    }
                }
                Opcode::Eqs => {
                    let lhs = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[0],
                        instruction.info.args_mode[0],
                    );
                    let rhs = Computer::resolve_argument(
                        &intcode_data,
                        instruction.args[1],
                        instruction.info.args_mode[1],
                    );
                    let dst = instruction.args[2];

                    if lhs == rhs {
                        intcode_data[dst as usize] = 1;
                    } else {
                        intcode_data[dst as usize] = 0;
                    }
                }
                Opcode::Hlt => {
                    self.state = ProgramState::Finished;
                }
                Opcode::Nop => {}
            }
        }

        intcode_data
    }

    pub fn find_correct_noun_and_verb(&mut self, program_file: &String) -> (i32, i32) {
        let mut intcode_data = IntCodeParser::parse_input(program_file);
        for n in 0..=99 {
            for v in 0..=99 {
                intcode_data[1] = n;
                intcode_data[2] = v;

                let result_intcode = self.execute_intcode(intcode_data.clone());

                if result_intcode[0] == 19690720 {
                    return (n, v);
                }
                self.reset();
            }
        }

        panic!("Noun and verb resulting in 19690720 could not be found!");
    }

    pub fn thermal_environment_supervision_terminal_diagnostic_program(
        &mut self,
        program_file: String,
    ) {
        let intcode_data = IntCodeParser::parse_input(&program_file);
        self.execute_intcode(intcode_data);
    }

    pub fn run<F, R>(&mut self, program: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let result = program(self);
        self.reset();

        result
    }
}
