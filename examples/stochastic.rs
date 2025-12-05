mod copy_machine;

use copy_machine::*;
use intro_to_ee_and_cs::state_machines::{StochasticMachine, run};

fn main() {
    let copy_machine_model = copy_machine_model();
    let copy_machine = StochasticMachine::new(copy_machine_model);
    let output: Vec<_> = run(&copy_machine, &[Input::Copy; 20]).collect();
    println!("Output: {output:?}");
}
