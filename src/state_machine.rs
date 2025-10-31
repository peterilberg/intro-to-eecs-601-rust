use std::fmt::Display;

pub trait StateMachine<I, O, S> {
    fn get_start_state(&self) -> S;
    fn get_next_state(&self, state: &S, input: &I) -> (S, O);
}

pub fn run<I, O, S>(
    state_machine: &dyn StateMachine<I, O, S>,
    inputs: &[I],
) -> Vec<O> {
    get_trajectory(state_machine, state_machine.get_start_state(), inputs)
        .into_iter()
        .map(|Transition { output, .. }| output)
        .collect()
}

pub fn trace<I, O, S>(
    state_machine: &dyn StateMachine<I, O, S>,
    inputs: &[I],
) -> Vec<O>
where
    I: Display,
    O: Display,
    S: Display,
{
    let start_state = state_machine.get_start_state();
    println!("Start state: {start_state}");

    get_trajectory(state_machine, start_state, inputs)
        .into_iter()
        .map(
            |Transition {
                 i,
                 input,
                 output,
                 new_state,
             }| {
                print!("{i}: input {input} produces {output}");
                println!(" with new state: {new_state}");
                output
            },
        )
        .collect()
}

#[derive(Debug)]
pub struct Transition<'i, I, O, S> {
    pub i: usize,
    pub input: &'i I,
    pub output: O,
    pub new_state: S,
}

impl<'i, I, O, S> PartialEq for Transition<'i, I, O, S>
where
    I: PartialEq,
    O: PartialEq,
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
            && *self.input == *other.input
            && self.output == other.output
            && self.new_state == other.new_state
    }
}

pub fn get_trajectory<'i, I, O, S>(
    state_machine: &dyn StateMachine<I, O, S>,
    start_state: S,
    inputs: &'i [I],
) -> Vec<Transition<'i, I, O, S>> {
    let mut trajectory = Vec::new();
    let mut state = &start_state;
    for (i, input) in inputs.iter().enumerate() {
        let (new_state, output) = state_machine.get_next_state(state, input);
        trajectory.push(Transition {
            i,
            input,
            output,
            new_state,
        });
        state = &trajectory.last().unwrap().new_state;
    }
    trajectory
}
