use intro_to_ee_and_cs::state_machines::{Accumulator, trace};

fn main() {
    let accumulator = Accumulator::new(0);
    let output: Vec<_> = trace(&accumulator, &[100, -3, 4, -123, 10]).collect();
    println!("Output: {output:?}");
}
