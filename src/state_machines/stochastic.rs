use crate::{
    conditional_distribution::ConditionalDistribution,
    discrete_distribution::DiscreteDistribution, state_machine::StateMachine,
};
use std::hash::Hash;

pub struct Stochastic<I, O, S>
where
    I: Copy + Eq + Hash,
    O: Copy + Eq + Hash,
    S: Copy + Eq + Hash,
{
    initial_state: DiscreteDistribution<S>,
    transition: ConditionalDistribution<(S, I), S>,
    observation: ConditionalDistribution<S, O>,
}

impl<I, O, S> Stochastic<I, O, S>
where
    I: Copy + Eq + Hash,
    O: Copy + Eq + Hash,
    S: Copy + Eq + Hash,
{
    pub fn new(
        initial_state: DiscreteDistribution<S>,
        transition: ConditionalDistribution<(S, I), S>,
        observation: ConditionalDistribution<S, O>,
    ) -> Stochastic<I, O, S> {
        Stochastic {
            initial_state,
            transition,
            observation,
        }
    }

    pub fn initial_state(&self) -> &DiscreteDistribution<S> {
        &self.initial_state
    }

    pub fn transition(&self) -> &ConditionalDistribution<(S, I), S> {
        &self.transition
    }

    pub fn observation(&self) -> &ConditionalDistribution<S, O> {
        &self.observation
    }
}

impl<I, O, S> StateMachine<I, O, S> for Stochastic<I, O, S>
where
    I: Copy + Eq + Hash,
    O: Copy + Eq + Hash,
    S: Copy + Eq + Hash,
{
    fn get_start_state(&self) -> S {
        *self.initial_state.draw()
    }

    fn get_next_state(&self, state: &S, input: &I) -> (S, O) {
        let next_state = self.transition.when(&(*state, *input)).draw();
        let output = self.observation.when(state).draw();

        (*next_state, *output)
    }
}
