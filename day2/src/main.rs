use crate::computer::cpu::Computer;

mod computer;
mod parser;

fn main() {
    let mut computer = Computer::new();
    let program_file = "day2/data/day3_input.txt".to_string();

    // computer.run(computer::ProgramName::SimpleIntCode);
    // computer.run(computer::ProgramName::SearchNounAndVerb);
    computer.run(computer::ProgramName::TEST, program_file);
}
