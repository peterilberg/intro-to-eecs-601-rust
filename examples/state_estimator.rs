mod copy_machine;

use copy_machine::*;
use intro_to_ee_and_cs::state_machines::{StateEstimator, run};

fn main() {
    let copy_machine_model = copy_machine_model();
    let copy_machine = StateEstimator::new(copy_machine_model);
    let observations = [
        (Input::Copy, Output::Perfect),
        (Input::Copy, Output::Smudged),
    ];

    let output: Vec<_> = run(&copy_machine, &observations).collect();
    println!("Output: {output:?}");
}
