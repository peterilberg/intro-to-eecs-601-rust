use std::fmt::Display;

pub trait StateMachine<S, I, O> {
    fn get_start_state(&self) -> S;
    fn get_next_state(&self, state: S, input: &I) -> (S, O);
}

pub fn run<S, I, O>(
    state_machine: &dyn StateMachine<S, I, O>,
    inputs: &[I],
) -> Vec<O> {
    let mut outputs = Vec::new();
    transduce(
        state_machine,
        inputs,
        |_state| {},
        |_i, _input, output, _state| {
            outputs.push(output);
        },
    );
    outputs
}

pub fn trace<S, I, O>(
    state_machine: &dyn StateMachine<S, I, O>,
    inputs: &[I],
) -> Vec<O>
where
    S: Display,
    I: Display,
    O: Display,
{
    let mut outputs = Vec::new();
    transduce(
        state_machine,
        inputs,
        |state| {
            println!("Start state: {state}");
        },
        |i, input, output, state| {
            print!("{i}: input {input} produces {output}");
            println!(" with new state: {state}");
            outputs.push(output);
        },
    );
    outputs
}

fn transduce<S, I, O, FS, FN>(
    state_machine: &dyn StateMachine<S, I, O>,
    inputs: &[I],
    mut start_state_fn: FS,
    mut next_state_fn: FN,
) where
    FS: FnMut(&S),
    FN: FnMut(usize, &I, O, &S),
{
    let start_state = state_machine.get_start_state();
    start_state_fn(&start_state);

    inputs
        .iter()
        .enumerate()
        .fold(start_state, |state, (i, input)| {
            let (next_state, output) =
                state_machine.get_next_state(state, input);
            next_state_fn(i, input, output, &next_state);
            next_state
        });
}
