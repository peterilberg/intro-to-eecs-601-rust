use crate::state_machine::StateMachine;

pub struct Accumulator<T> {
    initial_value: T,
}

impl<T> Accumulator<T> {
    pub fn new(initial_value: T) -> Accumulator<T> {
        Accumulator { initial_value }
    }
}

impl StateMachine<i32, i32, i32> for Accumulator<i32> {
    fn get_start_state(&self) -> i32 {
        self.initial_value
    }

    fn get_next_state(&self, state: i32, input: &i32) -> (i32, i32) {
        (state + input, state + input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::{get_trajectory, run};

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
                (0, 100, 100, 100),
                (1, -3, 97, 97),
                (2, 4, 101, 101),
                (3, -123, -22, -22),
                (4, 10, -12, -12),
            ]
        );
    }
}
