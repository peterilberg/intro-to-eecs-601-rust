use crate::state_machine::StateMachine;
use std::ops::Add;

pub struct Accumulator<T> {
    initial_value: T,
}

impl<T> Accumulator<T> {
    pub fn new(initial_value: T) -> Accumulator<T> {
        Accumulator { initial_value }
    }
}

impl<T> StateMachine<T, T, T> for Accumulator<T>
where
    T: Clone,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    fn get_start_state(&self) -> T {
        self.initial_value.clone()
    }

    fn get_next_state(&self, state: &T, input: &T) -> (T, T) {
        (state + input, state + input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::{Transition, get_trajectory, run};

    #[test]
    fn run_with_example_inputs() {
        let accumulator = Accumulator::new(0);
        let inputs = &[100, -3, 4, -123, 10];
        let output = run(&accumulator, inputs);
        assert_eq!(output, [100, 97, 101, -22, -12]);
    }

    #[test]
    fn get_trajectory_for_example_inputs() {
        let accumulator = Accumulator::new(0);
        let start_state = accumulator.get_start_state();
        let inputs = &[100, -3, 4, -123, 10];
        let trajectory = get_trajectory(&accumulator, start_state, inputs);
        assert_eq!(
            trajectory,
            [
                Transition {
                    i: 0,
                    input: &inputs[0],
                    output: 100,
                    new_state: 100
                },
                Transition {
                    i: 1,
                    input: &inputs[1],
                    output: 97,
                    new_state: 97
                },
                Transition {
                    i: 2,
                    input: &inputs[2],
                    output: 101,
                    new_state: 101
                },
                Transition {
                    i: 3,
                    input: &inputs[3],
                    output: -22,
                    new_state: -22
                },
                Transition {
                    i: 4,
                    input: &inputs[4],
                    output: -12,
                    new_state: -12
                },
            ]
        );
    }
}
