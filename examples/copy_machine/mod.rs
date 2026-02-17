use intro_to_ee_and_cs::distributions::Discrete;
use intro_to_ee_and_cs::state_machines::{
    StochasticModel, StochasticObservationModel, StochasticStateModel,
    StochasticTransitionModel,
};

#[derive(Clone, Copy, Debug)]
pub enum Input {
    Copy,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Output {
    Perfect,
    Smudged,
    Black,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum State {
    Good,
    Bad,
}

struct Model {}

impl StochasticModel for Model {
    type Input = Input;
    type Output = Output;
    type State = State;

    fn initial_state(&self) -> StochasticStateModel<Self::State> {
        Discrete::from([(State::Good, 0.9), (State::Bad, 0.1)])
    }

    fn observation<'model>(
        &'model self,
    ) -> StochasticObservationModel<'model, Self::State, Self::Output> {
        &|given: &Self::State| match *given {
            State::Good => Discrete::from([
                (Output::Perfect, 0.8),
                (Output::Smudged, 0.1),
                (Output::Black, 0.1),
            ]),
            State::Bad => Discrete::from([
                (Output::Perfect, 0.1),
                (Output::Smudged, 0.7),
                (Output::Black, 0.2),
            ]),
        }
    }

    fn transition<'model>(
        &'model self,
        input: &Self::Input,
    ) -> StochasticTransitionModel<'model, Self::State> {
        match *input {
            Input::Copy => &|given: &State| match *given {
                State::Good => {
                    Discrete::from([(State::Good, 0.7), (State::Bad, 0.3)])
                }
                State::Bad => {
                    Discrete::from([(State::Good, 0.1), (State::Bad, 0.9)])
                }
            },
        }
    }
}

pub fn copy_machine_model()
-> impl StochasticModel<Input = Input, Output = Output, State = State> {
    Model {}
}
