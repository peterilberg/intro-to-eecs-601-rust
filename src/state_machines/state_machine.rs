//! State machines.

use std::fmt::Display;

/// State machines must implement the `StateMachine` trait.
pub trait StateMachine<Input> {
    type Output;
    type State;

    /// Get the start state of the state machine.
    fn get_start_state(&self) -> Self::State;

    /// Get the next state of the state machine.
    fn get_next_state(
        &self,
        state: &Self::State,
        input: &Input,
    ) -> (Self::State, Self::Output);
}

type Machine<I, O, S> = dyn StateMachine<I, Output = O, State = S>;

/// An execution of a state machine.
pub struct Execution<'m, I, O, S> {
    state_machine: &'m Machine<I, O, S>,
    current_state: S,
}

impl<'m, I, O, S> Execution<'m, I, O, S> {
    /// Create a new execution from a state machine.
    fn new(state_machine: &'m Machine<I, O, S>) -> Self {
        Execution {
            state_machine,
            current_state: state_machine.get_start_state(),
        }
    }

    /// Advance the state machine by passing the input. Return the output.
    fn step(&mut self, input: &I) -> O {
        let (new_state, output) = self
            .state_machine
            .get_next_state(&self.current_state, input);
        self.current_state = new_state;
        output
    }
}

/// Run a state machine on a list of inputs. Return the outputs at every step.
pub fn run<I, O, S>(
    state_machine: &Machine<I, O, S>,
    inputs: &[I],
) -> impl Iterator<Item = O> {
    let mut execution = Execution::new(state_machine);
    inputs.iter().map(move |input| execution.step(input))
}

/// Run a state machine on a list of inputs. Return the outputs at every step.
/// Display a trace of the execution with all state transitions.
pub fn trace<I, O, S>(
    state_machine: &Machine<I, O, S>,
    inputs: &[I],
) -> impl Iterator<Item = O>
where
    I: Display,
    O: Display,
    S: Display,
{
    let mut execution = Execution::new(state_machine);
    println!("Start state: {}", execution.current_state);

    inputs.iter().enumerate().map(move |(i, input)| {
        let output = execution.step(input);
        print!("{i}: input {input} produces {output}");
        println!(" with new state: {}", execution.current_state);
        output
    })
}

/// A state transition contains the old state, the input that triggered the
/// transition, the output of the transition, and the new state. The number
/// `i` indicates when the transition was made during execution.
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
    /// Compare two state transitions.
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
            && self.old_state == other.old_state
            && self.input == other.input
            && self.output == other.output
            && self.new_state == other.new_state
    }
}

/// Run a state machine on a list of inputs. Return the state transitions.
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
