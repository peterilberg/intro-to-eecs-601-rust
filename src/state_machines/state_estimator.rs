use crate::{
    discrete_distribution::DiscreteDistribution, state_machine::StateMachine,
    state_machines::Stochastic,
};
use std::hash::Hash;

pub struct StateEstimator<I, O, S>
where
    I: Copy + Eq + Hash,
    O: Copy + Eq + Hash,
    S: Copy + Eq + Hash,
{
    state_machine: Stochastic<I, O, S>,
}

impl<I, O, S> StateEstimator<I, O, S>
where
    I: Copy + Eq + Hash,
    O: Copy + Eq + Hash,
    S: Copy + Eq + Hash,
{
    pub fn new(state_machine: Stochastic<I, O, S>) -> StateEstimator<I, O, S> {
        StateEstimator { state_machine }
    }
}

impl<I, O, S>
    StateMachine<(I, O), DiscreteDistribution<S>, DiscreteDistribution<S>>
    for StateEstimator<I, O, S>
where
    I: Copy + Eq + Hash,
    O: Copy + Eq + Hash,
    S: Copy + Eq + Hash,
{
    fn get_start_state(&self) -> DiscreteDistribution<S> {
        self.state_machine.initial_state().clone()
    }

    fn get_next_state(
        &self,
        state: &DiscreteDistribution<S>,
        (action, observed): &(I, O),
    ) -> (DiscreteDistribution<S>, DiscreteDistribution<S>) {
        let estimated_state =
            state.bayes(self.state_machine.observation(), observed);
        let next_state = estimated_state
            .marginalize(|event| (*event, *action))
            .total_probability(self.state_machine.transition());
        (next_state, estimated_state)
    }
}
