trait StateMachine<S, I, O> {
    fn start(&mut self);
    fn step(&mut self, input: &I) -> O;
    fn get_next_state(&self, state: S, input: &I) -> (S, O);
    fn transduce(&mut self, inputs: &[I], verbose: bool) -> Vec<O> {
        self.start();
        inputs.iter().map(|i| self.step(i)).collect()
    }
}

struct Accumulator<T> {
    start_state: T,
    state: T,
}

impl StateMachine<i32, i32, i32> for Accumulator<i32> {
    fn start(&mut self) {
        self.state = self.start_state;
    }

    fn step(&mut self, input: &i32) -> i32 {
        let (state, output) = self.get_next_state(self.state, input);
        self.state = state;
        output
    }

    fn get_next_state(&self, state: i32, input: &i32) -> (i32, i32) {
        (state + input, state + input)
    }
}

fn main() {
    let mut accumulator = Accumulator {
        start_state: 0,
        state: 0,
    };
    let output = accumulator.transduce(&[100, -3, 4, -123, 10], true);
    println!("Output: {output:?}");
}

// Start state: 0
// In: 100 Out: 100 Next State: 100
// In: -3 Out: 97 Next State: 97
// In: 4 Out: 101 Next State: 101
// In: -123 Out: -22 Next State: -22
// In: 10 Out: -12 Next State: -12
// [100, 97, 101, -22, -12]
