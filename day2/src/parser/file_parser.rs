use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct IntCodeParser {}

impl IntCodeParser {
    pub fn parse_input(filename: String) -> Vec<i32> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let lines = reader.lines();

        let mut intcode: Vec<i32> = Vec::new();
        for line in lines {
            for code in line.unwrap().split(",") {
                intcode.push(code.parse::<i32>().unwrap());
            }
        }

        intcode
    }
}
