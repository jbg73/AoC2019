use crate::computer::cpu::Computer;

mod computer;
mod parser;

fn main() {
    let mut computer = Computer::new();
    computer.load_program_from_file("day2/data/input.txt".to_string());

    computer.run(computer::ProgramName::SimpleIntCode);
    computer.run(computer::ProgramName::SearchNounAndVerb);
}
