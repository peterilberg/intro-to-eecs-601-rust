use std::fmt::Display;

trait StateMachine<S, I, O> {
    fn get_start_state(&self) -> S;
    fn get_next_state(&self, state: S, input: &I) -> (S, O);

    fn run(&self, inputs: &[I]) -> Vec<O> {
        let mut outputs = Vec::new();
        self.transduce(
            inputs,
            |_state| {},
            |_i, _input, output, _state| {
                outputs.push(output);
            },
        );
        outputs
    }

    fn trace(&self, inputs: &[I]) -> Vec<O>
    where
        S: Display,
        I: Display,
        O: Display,
    {
        let mut outputs = Vec::new();
        self.transduce(
            inputs,
            |state| {
                println!("Start state: {state}");
            },
            |i, input, output, state| {
                println!("{i}: input {input} produces {output} with new state: {state}");
                outputs.push(output);
            },
        );
        outputs
    }

    fn transduce<FS, FN>(&self, inputs: &[I], mut start_state_fn: FS, mut next_state_fn: FN)
    where
        FS: FnMut(&S),
        FN: FnMut(usize, &I, O, &S),
    {
        let start_state = self.get_start_state();
        start_state_fn(&start_state);

        inputs
            .iter()
            .enumerate()
            .fold(start_state, |state, (i, input)| {
                let (next_state, output) = self.get_next_state(state, input);
                next_state_fn(i, input, output, &next_state);
                next_state
            });
    }
}

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
    let output = accumulator.trace(&[100, -3, 4, -123, 10]);
    println!("Output: {output:?}");
}

// Start state: 0
// In: 100 Out: 100 Next State: 100
// In: -3 Out: 97 Next State: 97
// In: 4 Out: 101 Next State: 101
// In: -123 Out: -22 Next State: -22
// In: 10 Out: -12 Next State: -12
// [100, 97, 101, -22, -12]
