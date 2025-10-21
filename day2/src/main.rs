use crate::computer::cpu::Computer;

mod computer;
mod parser;

fn main() {
    let mut computer = Computer::new();
    let noun_and_verb_file = "day2/data/input.txt".to_string();
    let test_program_file = "day2/data/day3_input.txt".to_string();

    let res = computer.run(|cpu| cpu.find_correct_noun_and_verb(&noun_and_verb_file));
    println!("Result from Find N&V: {:?} \n", res);

    computer.run(|cpu| {
        cpu.thermal_environment_supervision_terminal_diagnostic_program(test_program_file)
    });
}
