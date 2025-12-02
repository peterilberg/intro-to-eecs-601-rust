use intro_to_ee_and_cs::conditional_distribution::ConditionalDistribution;
use intro_to_ee_and_cs::discrete_distribution::DiscreteDistribution;
use intro_to_ee_and_cs::state_machines::Stochastic;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum State {
    Good,
    Bad,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Input {
    Copy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Output {
    Perfect,
    Smudged,
    Black,
}

pub fn copy_machine() -> Stochastic<Input, Output, State> {
    let initial_state =
        DiscreteDistribution::from([(State::Good, 0.9), (State::Bad, 0.1)]);

    let observation = ConditionalDistribution::from([
        (
            State::Good,
            DiscreteDistribution::from([
                (Output::Perfect, 0.8),
                (Output::Smudged, 0.1),
                (Output::Black, 0.1),
            ]),
        ),
        (
            State::Bad,
            DiscreteDistribution::from([
                (Output::Perfect, 0.1),
                (Output::Smudged, 0.7),
                (Output::Black, 0.2),
            ]),
        ),
    ]);

    let transition = ConditionalDistribution::from([
        (
            (State::Good, Input::Copy),
            DiscreteDistribution::from([(State::Good, 0.7), (State::Bad, 0.3)]),
        ),
        (
            (State::Bad, Input::Copy),
            DiscreteDistribution::from([(State::Good, 0.1), (State::Bad, 0.9)]),
        ),
    ]);

    Stochastic::new(initial_state, transition, observation)
}
