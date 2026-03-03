//! The stochastic model underlying a stochastic state machine.

use crate::{distributions::Conditional, distributions::Discrete};
use std::hash::Hash;

/// The stochastic state model is a probability distribution over states.
pub type StochasticStateModel<State> = Discrete<State>;

/// The stochastic transition model is a conditional distribution from
/// old state to new state.
pub type StochasticTransitionModel<'model, State> =
    &'model Conditional<State, State>;

/// The stochastic observation model is a conditional distribution from
/// old state to output.
pub type StochasticObservationModel<'model, State, Output> =
    &'model Conditional<State, Output>;

/// The stochastic model defines distributions for the initial state,
/// the state transitions, and the observations in each state.
pub trait StochasticModel {
    type Input;
    type Output: Clone + Eq + Hash;
    type State: Clone + Eq + Hash;

    /// The initial state distribution.
    fn initial_state(&self) -> StochasticStateModel<Self::State>;

    /// The conditional transition distribution depending on input and state.
    fn transition<'model>(
        &'model self,
        input: &Self::Input,
    ) -> StochasticTransitionModel<'model, Self::State>;

    // The conditional observation distribution depending on the state.
    fn observation<'model>(
        &'model self,
    ) -> StochasticObservationModel<'model, Self::State, Self::Output>;
}
