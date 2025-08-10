use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct IntCodeParser {
    data: Vec<i32>,
}

impl IntCodeParser {
    fn parse_input(filename: String) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let lines = reader.lines();

        let mut intcode: Vec<i32> = Vec::new();
        for line in lines {
            for code in line.unwrap().split(",") {
                intcode.push(code.parse::<i32>().unwrap());
            }
        }

        Self { data: intcode }
    }
}

struct Computer {
    intcode_parser: IntCodeParser,
}

impl Computer {
    fn new(intcode_parser: IntCodeParser) -> Self {
        Self { intcode_parser }
    }

    fn add2(val_l: i32, val_r: i32) -> i32 {
        val_l + val_r
    }

    fn multiply2(val_l: i32, val_r: i32) -> i32 {
        val_l * val_r
    }

    fn execute_intcode(&mut self) -> Vec<i32> {
        let mut result_intcode = self.intcode_parser.data.clone();
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
                self.intcode_parser.data[1] = n;
                self.intcode_parser.data[2] = v;

                let result_intcode = self.execute_intcode();

                if result_intcode[0] == 19690720 {
                    return (n, v);
                }
            }
        }

        panic!("Noun and verb resulting in 19690720 could not be found!");
    }
}

fn main() {
    let intcode_parser = IntCodeParser::parse_input("day2/data/input.txt".to_string());
    let mut computer = Computer::new(intcode_parser);

    let result_intcode = computer.execute_intcode();
    println!("Final Intcode[0]: {}", result_intcode[0]);

    let (n, v) = computer.find_correct_noun_and_verb();
    println!("Noun: {n}, Verb: {v}");
}
