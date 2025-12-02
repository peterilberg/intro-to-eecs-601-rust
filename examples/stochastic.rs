mod copy_machine;

use copy_machine::*;
use intro_to_ee_and_cs::state_machine::run;

fn main() {
    let copy_machine = copy_machine();
    let output: Vec<_> = run(&copy_machine, &[Input::Copy; 20]).collect();
    println!("Output: {output:?}");
}
