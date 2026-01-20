use crate::{state_machines::StateMachine, state_machines::StochasticModel};
use std::hash::Hash;

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

    fn get_start_state(&self) -> Self::State {
        self.model.initial_state().draw().clone()
    }

    fn get_next_state(
        &self,
        state: &Self::State,
        input: &I,
    ) -> (Self::State, Self::Output) {
        let output = self.model.observation().when(state).draw();
        let next_state = self.model.transition(input).when(state).draw();

        (next_state.clone(), output.clone())
    }
}
