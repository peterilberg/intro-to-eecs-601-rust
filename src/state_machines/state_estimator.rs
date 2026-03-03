//! State estimation state machines.

use crate::{
    distributions::Discrete, distributions::bayesian_evidence,
    distributions::total_probability, state_machines::StateMachine,
    state_machines::StochasticModel,
};
use std::hash::Hash;

/// A state estimation state machine.
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
    /// Create a new state estimator from a stochastic model.
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

    /// The start state is the stochastic model's initial state distribution.
    fn get_start_state(&self) -> Self::State {
        self.model.initial_state()
    }

    /// `get_next_state` returns the next state and the estimated state.
    ///
    /// The estimated state is a probability distribution over all states after
    /// applying a Bayesian inference step on the stochastic model's
    /// observation distribution with prior old state distribution and
    /// `observed` evidence.
    ///
    /// The next state is a probability distribution over all states after
    /// computing the total probability on the stochastic model's
    /// transition distribution with prior estimated state and given `action`.
    fn get_next_state(
        &self,
        state: &Self::State,
        (action, observed): &(I, O),
    ) -> (Self::State, Self::Output) {
        let observation = self.model.observation();
        let transition = self.model.transition(action);

        let estimated_state = bayesian_evidence(observation, state, observed);
        let next_state = total_probability(transition, &estimated_state);

        (next_state, estimated_state)
    }
}
