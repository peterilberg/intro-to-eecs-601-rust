mod copy_machine;

use copy_machine::*;
use intro_to_ee_and_cs::state_machine::run;
use intro_to_ee_and_cs::state_machines::StateEstimator;

fn main() {
    let copy_machine = copy_machine();
    let estimator = StateEstimator::new(copy_machine);
    let observations = [
        (Input::Copy, Output::Perfect),
        (Input::Copy, Output::Smudged),
    ];

    let output: Vec<_> = run(&estimator, &observations).collect();
    println!("Output: {output:?}");
}
