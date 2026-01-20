use std::fmt::Display;

pub trait StateMachine<Input> {
    type Output;
    type State;

    fn get_start_state(&self) -> Self::State;
    fn get_next_state(
        &self,
        state: &Self::State,
        input: &Input,
    ) -> (Self::State, Self::Output);
}

type Machine<I, O, S> = dyn StateMachine<I, Output = O, State = S>;

pub struct Execution<'m, I, O, S> {
    state_machine: &'m Machine<I, O, S>,
    current_state: S,
}

impl<'m, I, O, S> Execution<'m, I, O, S> {
    fn new(state_machine: &'m Machine<I, O, S>) -> Self {
        Execution {
            state_machine,
            current_state: state_machine.get_start_state(),
        }
    }

    fn step(&mut self, input: &I) -> O {
        let (new_state, output) = self
            .state_machine
            .get_next_state(&self.current_state, input);
        self.current_state = new_state;
        output
    }
}

pub fn run<I, O, S>(
    state_machine: &Machine<I, O, S>,
    inputs: &[I],
) -> impl Iterator<Item = O> {
    let mut execution = Execution::new(state_machine);
    inputs.iter().map(move |input| execution.step(input))
}

pub fn trace<I, O, S>(
    state_machine: &Machine<I, O, S>,
    inputs: &[I],
) -> impl Iterator<Item = O>
where
    I: Display,
    O: Display,
    S: Display,
{
    let mut exeuction = Execution::new(state_machine);
    println!("Start state: {}", exeuction.current_state);

    inputs.iter().enumerate().map(move |(i, input)| {
        let output = exeuction.step(input);
        print!("{i}: input {input} produces {output}");
        println!(" with new state: {}", exeuction.current_state);
        output
    })
}

#[derive(Debug)]
pub struct Transition<I, O, S> {
    pub i: usize,
    pub old_state: S,
    pub input: I,
    pub output: O,
    pub new_state: S,
}

impl<I, O, S> PartialEq for Transition<I, O, S>
where
    I: PartialEq,
    O: PartialEq,
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
            && self.old_state == other.old_state
            && self.input == other.input
            && self.output == other.output
            && self.new_state == other.new_state
    }
}

pub fn get_transitions<I, O, S>(
    state_machine: &Machine<I, O, S>,
    inputs: &[I],
) -> impl Iterator<Item = Transition<I, O, S>>
where
    I: Clone,
    S: Clone,
{
    let mut execution = Execution::new(state_machine);
    inputs.iter().enumerate().map(move |(i, input)| {
        let old_state = execution.current_state.clone();
        let output = execution.step(input);
        Transition {
            i,
            old_state,
            input: input.clone(),
            output,
            new_state: execution.current_state.clone(),
        }
    })
}
