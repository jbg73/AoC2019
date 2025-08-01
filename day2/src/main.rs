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

    fn get_intcode(self) -> Vec<i32> {
        self.intcode_parser.data
    }

    //fn add2(mut self, val_l: i32, val_r: i32, dst_idx: usize) {
    //    let addition_result = val_l + val_r;
    //    self.intcode_parser.data[dst_idx] = addition_result;
    //}

    fn add2(val_l: i32, val_r: i32) -> i32 {
        val_l + val_r
    }

    //fn multiply2(mut self, val_l: i32, val_r: i32, result_idx: usize) {
    //  let multiplication_result = val_l * val_r;
    //  self.intcode_parser.data[dst_idx] = multiplication_result;
    //}

    fn multiply2(val_l: i32, val_r: i32) -> i32 {
        val_l * val_r
    }

    fn execute_intcode(&mut self) {
        println!("Intcode data: {:?}", self.intcode_parser.data);
        let mut idx = 0;
        let mut code = Some(self.intcode_parser.data[idx]);
        while let Some(value) = code {
            if value == 99 {
                code = None;
            } else {
                match value {
                    1 => {
                        let sum = Self::add2(
                            self.intcode_parser.data[self.intcode_parser.data[idx + 1] as usize],
                            self.intcode_parser.data[self.intcode_parser.data[idx + 2] as usize],
                        );
                        let dst_idx = self.intcode_parser.data[idx + 3] as usize;
                        self.intcode_parser.data[dst_idx] = sum;
                        idx += 4;
                    }
                    2 => {
                        let multiplication = Self::multiply2(
                            self.intcode_parser.data[self.intcode_parser.data[idx + 1] as usize],
                            self.intcode_parser.data[self.intcode_parser.data[idx + 2] as usize],
                        );
                        let dst_idx = self.intcode_parser.data[idx + 3] as usize;
                        self.intcode_parser.data[dst_idx] = multiplication;
                        idx += 4;
                    }
                    _ => idx += 1,
                }
                code = Some(self.intcode_parser.data[idx]);
            }
        }
    }
}

fn main() {
    let intcode_parser = IntCodeParser::parse_input("day2/data/input.txt".to_string());
    let mut computer = Computer::new(intcode_parser);

    computer.execute_intcode();

    let result = computer.get_intcode();
    println!("Final Intcode: {:?}", result);
}
