//! Stochastic state machines.

use crate::{state_machines::StateMachine, state_machines::StochasticModel};
use std::hash::Hash;

/// A stochastic state machine.
pub struct StochasticMachine<I, O, S, M>
where
    O: Clone + Eq + Hash,
    S: Clone + Eq + Hash,
    M: StochasticModel<Input = I, Output = O, State = S>,
{
    model: M,
}

impl<I, O, S, M> StochasticMachine<I, O, S, M>
where
    O: Clone + Eq + Hash,
    S: Clone + Eq + Hash,
    M: StochasticModel<Input = I, Output = O, State = S>,
{
    /// Create a new stochastic state machine from a stochastic model.
    pub fn new(model: M) -> StochasticMachine<I, O, S, M> {
        StochasticMachine { model }
    }
}

impl<I, O, S, M> StateMachine<I> for StochasticMachine<I, O, S, M>
where
    O: Clone + Eq + Hash,
    S: Clone + Eq + Hash,
    M: StochasticModel<Input = I, Output = O, State = S>,
{
    type Output = O;
    type State = S;

    /// The start state is randomly drawn from the stochastic model's
    /// initial state distribution.
    fn get_start_state(&self) -> Self::State {
        self.model.initial_state().draw().clone()
    }

    /// The next state and the corresponding output and randomly drawn
    /// from the stochasstic model's transition and observation
    /// distributions based on the old state and the `input`.
    fn get_next_state(
        &self,
        state: &Self::State,
        input: &I,
    ) -> (Self::State, Self::Output) {
        let observation = self.model.observation()(state);
        let transition = self.model.transition(input)(state);

        let output = observation.draw();
        let next_state = transition.draw();

        (next_state.clone(), output.clone())
    }
}
