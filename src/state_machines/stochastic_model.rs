use crate::{distributions::Conditional, distributions::Discrete};
use std::hash::Hash;

pub type StochasticStateModel<State> = Discrete<State>;
pub type StochasticTransitionModel<'model, State> =
    &'model Conditional<State, State>;
pub type StochasticObservationModel<'model, State, Output> =
    &'model Conditional<State, Output>;

pub trait StochasticModel {
    type Input;
    type Output: Clone + Eq + Hash;
    type State: Clone + Eq + Hash;

    fn initial_state(&self) -> StochasticStateModel<Self::State>;

    fn transition<'model>(
        &'model self,
        input: &Self::Input,
    ) -> StochasticTransitionModel<'model, Self::State>;

    fn observation<'model>(
        &'model self,
    ) -> StochasticObservationModel<'model, Self::State, Self::Output>;
}
