use std::fmt::Display;

pub trait StateMachine {
    type Input;
    type Output;
    type State;

    fn get_start_state(&self) -> Self::State;
    fn get_next_state(
        &self,
        state: &Self::State,
        input: &Self::Input,
    ) -> (Self::State, Self::Output);
}

type Machine<I, O, S> = dyn StateMachine<Input = I, Output = O, State = S>;

pub fn run<I, O, S>(
    state_machine: &Machine<I, O, S>,
    inputs: &[I],
) -> impl Iterator<Item = O> {
    let mut stepper = Stepper::new(state_machine);
    inputs.iter().map(move |input| stepper.step(input))
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
    let mut stepper = Stepper::new(state_machine);
    println!("Start state: {}", stepper.current_state);

    inputs.iter().enumerate().map(move |(i, input)| {
        let output = stepper.step(input);
        print!("{i}: input {input} produces {output}");
        println!(" with new state: {}", stepper.current_state);
        output
    })
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
    state_machine: &Machine<I, O, S>,
    inputs: &'i [I],
) -> impl Iterator<Item = Transition<'i, I, O, S>>
where
    S: Clone,
{
    let mut stepper = Stepper::new(state_machine);
    inputs.iter().enumerate().map(move |(i, input)| {
        let output = stepper.step(input);
        Transition {
            i,
            input,
            output,
            new_state: stepper.current_state.clone(),
        }
    })
}

struct Stepper<'m, I, O, S> {
    state_machine: &'m Machine<I, O, S>,
    current_state: S,
}

impl<'m, I, O, S> Stepper<'m, I, O, S> {
    fn new(state_machine: &'m Machine<I, O, S>) -> Self {
        Stepper {
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
