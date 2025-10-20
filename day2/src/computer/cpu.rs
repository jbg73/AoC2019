use std::result;

use super::{Instruction, InstructionInfo, ParameterMode, ProgramName, ProgramState};
use crate::parser::file_parser::IntCodeParser;

pub struct Computer {
    intcode_data: Vec<i32>,
    current_address: usize,
    state: ProgramState,
    input: i32,
}

impl Computer {
    // returns the number of "jumps" to do on the "sp"
    fn get_operation_count(opcode: i32) -> usize {
        match opcode {
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 1,
            _ => 0,
        }
    }

    fn read_instruction(mut opcode: i32) -> InstructionInfo {
        let opcode_parsed = opcode % 100;
        opcode = opcode / 100;

        let mut param_modes: Vec<ParameterMode> = Vec::new();
        while opcode > 10 {
            let mode = opcode % 10;
            let mode = match mode {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => ParameterMode::Position,
            };
            param_modes.push(mode);

            opcode = opcode / 10;
        }

        InstructionInfo {
            opcode: opcode_parsed,
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
            intcode_data: Vec::new(),
            current_address: 0,
            state: ProgramState::Idle,
            input: 1, // TODO: read problem and properly handle input. Not Hardcoded
        }
    }

    pub fn load_program_from_file(&mut self, filename: String) {
        self.intcode_data = IntCodeParser::parse_input(filename);
    }

    pub fn reset(&mut self) {
        self.current_address = 0;
        self.state = ProgramState::Idle;
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let next_intcode = self.intcode_data[self.current_address];
        let instruction = Computer::read_instruction(next_intcode);

        self.current_address += 1;

        let n_args = Computer::get_operation_count(instruction.opcode);

        let start_idx = self.current_address;
        let end_idx = start_idx + n_args;
        let fetched_args: Vec<i32> = self.intcode_data[start_idx..end_idx].to_vec();
        self.current_address += n_args;

        Instruction {
            info: instruction,
            args: fetched_args,
        }
    }

    fn execute_intcode(&mut self) -> Vec<i32> {
        let mut result_intcode = self.intcode_data.clone();
        self.state = ProgramState::Running;

        while (self.current_address < result_intcode.len()) && (self.state == ProgramState::Running)
        {
            let instruction = self.fetch_instruction();
            match instruction.info.opcode {
                1 => {
                    let operand1 = result_intcode[instruction.args[0] as usize];
                    let operand2 = result_intcode[instruction.args[1] as usize];
                    let dst = instruction.args[2];

                    let result = Computer::add2(operand1, operand2);
                    result_intcode[dst as usize] = result;
                }
                2 => {
                    let operand1 = result_intcode[instruction.args[0] as usize];
                    let operand2 = result_intcode[instruction.args[1] as usize];
                    let dst = instruction.args[2];

                    let result = Computer::multiply2(operand1, operand2);
                    result_intcode[dst as usize] = result;
                }
                3 => {
                    let dst = instruction.args[0];
                    result_intcode[dst as usize] = self.input;
                }
                4 => {
                    let src = instruction.args[0];
                    println!("{}", result_intcode[src as usize]);
                }
                99 => {
                    self.state = ProgramState::Finished;
                }
                _ => {}
            }
        }

        result_intcode
    }

    fn find_correct_noun_and_verb(&mut self) -> (i32, i32) {
        for n in 0..=99 {
            for v in 0..=99 {
                self.intcode_data[1] = n;
                self.intcode_data[2] = v;

                let result_intcode = self.execute_intcode();

                if result_intcode[0] == 19690720 {
                    return (n, v);
                }
                self.reset();
            }
        }

        panic!("Noun and verb resulting in 19690720 could not be found!");
    }

    pub fn run(&mut self, program: ProgramName) {
        match program {
            ProgramName::SimpleIntCode => {
                let result = self.execute_intcode();
                println!("Intcode program result: {}", result[0]);
            }
            ProgramName::SearchNounAndVerb => {
                let result = self.find_correct_noun_and_verb();
                println!("Search N&V program result: ({},{})", result.0, result.1);
            }
            ProgramName::TEST => {
                self.execute_intcode();
            }
        }
    }
}
