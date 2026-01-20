use crate::{
    distributions::Discrete, state_machines::StateMachine,
    state_machines::StochasticModel,
};
use std::hash::Hash;

pub struct StateEstimator<I, O, S, M>
where
    O: Clone + Eq + Hash,
    S: Clone + Eq + Hash,
    M: StochasticModel<Input = I, Output = O, State = S>,
{
    model: M,
}

impl<I, O, S, M> StateEstimator<I, O, S, M>
where
    O: Clone + Eq + Hash,
    S: Clone + Eq + Hash,
    M: StochasticModel<Input = I, Output = O, State = S>,
{
    pub fn new(model: M) -> StateEstimator<I, O, S, M> {
        StateEstimator { model }
    }
}

impl<I, O, S, M> StateMachine<(I, O)> for StateEstimator<I, O, S, M>
where
    O: Clone + Eq + Hash,
    S: Clone + Eq + Hash,
    M: StochasticModel<Input = I, Output = O, State = S>,
{
    type Output = Discrete<S>;
    type State = Discrete<S>;

    fn get_start_state(&self) -> Self::State {
        self.model.initial_state()
    }

    fn get_next_state(
        &self,
        state: &Self::State,
        (action, observed): &(I, O),
    ) -> (Self::State, Self::Output) {
        let observation = self.model.observation();
        let transition = self.model.transition(action);

        let estimated_state = observation.bayesian_evidence(state, observed);
        let next_state = transition.total_probability(&estimated_state);

        (next_state, estimated_state)
    }
}
