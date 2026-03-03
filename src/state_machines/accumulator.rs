//! An accumulator state machine.

use crate::state_machines::StateMachine;
use std::ops::Add;

/// An accumulator state machine.
pub struct Accumulator<T> {
    initial_value: T,
}

impl<T> Accumulator<T> {
    /// Create a new accumulator state machine with an initial value.
    pub fn new(initial_value: T) -> Accumulator<T> {
        Accumulator { initial_value }
    }
}

impl<T> StateMachine<T> for Accumulator<T>
where
    T: Clone,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Output = T;
    type State = T;

    /// The start state of an accumulator is the initial value.
    fn get_start_state(&self) -> Self::State {
        self.initial_value.clone()
    }

    /// The next state of an accumulator is the sum of the current
    /// state and the input.
    fn get_next_state(
        &self,
        state: &Self::State,
        input: &T,
    ) -> (Self::State, Self::Output) {
        (state + input, state + input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machines::{Transition, get_transitions, run};

    #[test]
    fn run_with_example_inputs() {
        let accumulator = Accumulator::new(0);
        let inputs = &[100, -3, 4, -123, 10];
        let output: Vec<_> = run(&accumulator, inputs).collect();
        assert_eq!(output, [100, 97, 101, -22, -12]);
    }

    #[test]
    fn get_trajectory_for_example_inputs() {
        let accumulator = Accumulator::new(0);
        let inputs = &[100, -3, 4, -123, 10];
        let trajectory: Vec<_> =
            get_transitions(&accumulator, inputs).collect();
        assert_eq!(
            trajectory,
            [
                Transition {
                    i: 0,
                    old_state: 0,
                    input: inputs[0],
                    output: 100,
                    new_state: 100
                },
                Transition {
                    i: 1,
                    old_state: 100,
                    input: inputs[1],
                    output: 97,
                    new_state: 97
                },
                Transition {
                    i: 2,
                    old_state: 97,
                    input: inputs[2],
                    output: 101,
                    new_state: 101
                },
                Transition {
                    i: 3,
                    old_state: 101,
                    input: inputs[3],
                    output: -22,
                    new_state: -22
                },
                Transition {
                    i: 4,
                    old_state: -22,
                    input: inputs[4],
                    output: -12,
                    new_state: -12
                },
            ]
        );
    }
}
