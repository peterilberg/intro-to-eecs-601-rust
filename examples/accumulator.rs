use intro_to_ee_and_cs::state_machine::trace;
use intro_to_ee_and_cs::state_machines::Accumulator;

fn main() {
    let accumulator = Accumulator::new(0);
    let output = trace(&accumulator, &[100, -3, 4, -123, 10]);
    println!("Output: {output:?}");
}
