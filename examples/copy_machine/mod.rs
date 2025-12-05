use intro_to_ee_and_cs::distributions::{Conditional, Discrete};
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

fn initial_state() -> Discrete<State> {
    Discrete::from([(State::Good, 0.9), (State::Bad, 0.1)])
}

impl ObservationModel {
    fn new() -> ObservationModel {
        ObservationModel {
            good: Discrete::from([
                (Output::Perfect, 0.8),
                (Output::Smudged, 0.1),
                (Output::Black, 0.1),
            ]),
            bad: Discrete::from([
                (Output::Perfect, 0.1),
                (Output::Smudged, 0.7),
                (Output::Black, 0.2),
            ]),
        }
    }
}

impl TransitionModel {
    fn new() -> TransitionModel {
        TransitionModel {
            good: Discrete::from([(State::Good, 0.7), (State::Bad, 0.3)]),
            bad: Discrete::from([(State::Good, 0.1), (State::Bad, 0.9)]),
        }
    }
}

pub struct ObservationModel {
    good: Discrete<Output>,
    bad: Discrete<Output>,
}

impl Conditional for ObservationModel {
    type Given = State;
    type Event = Output;

    fn when(&self, given: &Self::Given) -> &Discrete<Self::Event> {
        match *given {
            State::Good => &self.good,
            State::Bad => &self.bad,
        }
    }
}

pub struct TransitionModel {
    good: Discrete<State>,
    bad: Discrete<State>,
}

impl Conditional for TransitionModel {
    type Given = State;
    type Event = State;

    fn when(&self, given: &Self::Given) -> &Discrete<Self::Event> {
        match *given {
            State::Good => &self.good,
            State::Bad => &self.bad,
        }
    }
}

pub struct Model {
    transition: TransitionModel,
    observation: ObservationModel,
}

impl Model {
    fn new() -> Model {
        Model {
            transition: TransitionModel::new(),
            observation: ObservationModel::new(),
        }
    }
}

impl StochasticModel for Model {
    type Input = Input;
    type Output = Output;
    type State = State;

    fn initial_state(&self) -> StochasticStateModel<Self::State> {
        initial_state()
    }

    fn transition<'model>(
        &'model self,
        input: &Self::Input,
    ) -> StochasticTransitionModel<'model, Self::State> {
        match *input {
            Input::Copy => &self.transition,
        }
    }

    fn observation<'model>(
        &'model self,
    ) -> StochasticObservationModel<'model, Self::State, Self::Output> {
        &self.observation
    }
}

pub fn copy_machine_model()
-> impl StochasticModel<Input = Input, Output = Output, State = State> {
    Model::new()
}
