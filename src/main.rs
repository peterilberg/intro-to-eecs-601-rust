mod state_machine;

use state_machine::{StateMachine, trace};

struct Accumulator<T> {
    start_state: T,
}

impl StateMachine<i32, i32, i32> for Accumulator<i32> {
    fn get_start_state(&self) -> i32 {
        self.start_state
    }

    fn get_next_state(&self, state: i32, input: &i32) -> (i32, i32) {
        (state + input, state + input)
    }
}

fn main() {
    let accumulator = Accumulator { start_state: 0 };
    let output = trace(&accumulator, &[100, -3, 4, -123, 10]);
    println!("Output: {output:?}");
}
