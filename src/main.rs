mod state_machine;
mod state_machines;

use state_machine::trace;
use state_machines::accumulator::Accumulator;

fn main() {
    let accumulator = Accumulator::new(0);
    let output = trace(&accumulator, &[100, -3, 4, -123, 10]);
    println!("Output: {output:?}");
}
