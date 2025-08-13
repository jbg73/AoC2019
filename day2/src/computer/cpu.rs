use super::ProgramName;
use crate::parser::file_parser::IntCodeParser;

pub struct Computer {
    intcode_data: Vec<i32>,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            intcode_data: Vec::new(),
        }
    }

    pub fn load_program_from_file(&mut self, filename: String) {
        self.intcode_data = IntCodeParser::parse_input(filename);
    }

    fn add2(val_l: i32, val_r: i32) -> i32 {
        val_l + val_r
    }

    fn multiply2(val_l: i32, val_r: i32) -> i32 {
        val_l * val_r
    }

    fn execute_intcode(&mut self) -> Vec<i32> {
        let mut result_intcode = self.intcode_data.clone();
        let mut idx = 0;
        let mut code = Some(result_intcode[idx]);
        while let Some(value) = code {
            if value == 99 {
                code = None;
            } else {
                match value {
                    1 => {
                        let sum = Self::add2(
                            result_intcode[result_intcode[idx + 1] as usize],
                            result_intcode[result_intcode[idx + 2] as usize],
                        );
                        let dst_idx = result_intcode[idx + 3] as usize;
                        result_intcode[dst_idx] = sum;
                        idx += 4;
                    }
                    2 => {
                        let multiplication = Self::multiply2(
                            result_intcode[result_intcode[idx + 1] as usize],
                            result_intcode[result_intcode[idx + 2] as usize],
                        );
                        let dst_idx = result_intcode[idx + 3] as usize;
                        result_intcode[dst_idx] = multiplication;
                        idx += 4;
                    }
                    _ => idx += 1,
                }
                code = Some(result_intcode[idx]);
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
        }
    }
}
