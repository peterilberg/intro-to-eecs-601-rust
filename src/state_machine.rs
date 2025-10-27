use std::fmt::Display;

pub trait StateMachine<S, I, O> {
    fn get_start_state(&self) -> S;
    fn get_next_state(&self, state: S, input: &I) -> (S, O);
}

pub fn run<S, I, O>(
    state_machine: &dyn StateMachine<S, I, O>,
    inputs: &[I],
) -> Vec<O> {
    transduce(
        state_machine,
        state_machine.get_start_state(),
        inputs,
        |_i, _input, _output, _state| {},
    )
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
    let start_state = state_machine.get_start_state();
    println!("Start state: {start_state}");

    transduce(
        state_machine,
        state_machine.get_start_state(),
        inputs,
        |i, input, output, state| {
            print!("{i}: input {input} produces {output}");
            println!(" with new state: {state}");
        },
    )
}

pub fn get_trajectory<S, I, O>(
    state_machine: &dyn StateMachine<S, I, O>,
    start_state: S,
    inputs: &[I],
) -> Vec<(usize, I, O, S)>
where
    S: Clone,
    I: Clone,
    O: Clone,
{
    let mut trajectory = Vec::new();
    transduce(
        state_machine,
        start_state,
        inputs,
        |i, input, output, state| {
            trajectory.push((i, input.clone(), output.clone(), state.clone()));
        },
    );
    trajectory
}

fn transduce<S, I, O, F>(
    state_machine: &dyn StateMachine<S, I, O>,
    start_state: S,
    inputs: &[I],
    mut transition: F,
) -> Vec<O>
where
    F: FnMut(usize, &I, &O, &S),
{
    let mut outputs = Vec::new();

    inputs
        .iter()
        .enumerate()
        .fold(start_state, |state, (i, input)| {
            let (next_state, output) =
                state_machine.get_next_state(state, input);
            transition(i, input, &output, &next_state);
            outputs.push(output);
            next_state
        });

    outputs
}
