use crate::distributions::Discrete;
use core::hash::Hash;

pub type Conditional<Given, Event> = dyn Fn(&Given) -> Discrete<Event>;

pub fn join<Given, Event>(
    conditional: &Conditional<Given, Event>,
    distribution: &Discrete<Given>,
) -> Discrete<(Given, Event)>
where
    Given: Clone + Eq + Hash,
    Event: Clone + Eq + Hash,
{
    let joint_events = distribution.support().flat_map(|given| {
        let condition = distribution.probability(given);
        let conditioned = conditional(given);
        conditioned
            .support()
            .map(|event| {
                (
                    (given.clone(), event.clone()),
                    (condition * conditioned.probability(event)),
                )
            })
            .collect::<Vec<_>>()
    });
    Discrete::from_iter(joint_events)
}

pub fn bayesian_evidence<Given, Event>(
    conditional: &Conditional<Given, Event>,
    prior: &Discrete<Given>,
    evidence: &Event,
) -> Discrete<Given>
where
    Given: Clone + Eq + Hash,
    Event: Clone + Eq + Hash,
{
    join(conditional, prior)
        .condition(|(_, event)| event == evidence)
        .marginalize(|(event, _)| event.clone())
}

pub fn total_probability<Given, Event>(
    conditional: &Conditional<Given, Event>,
    prior: &Discrete<Given>,
) -> Discrete<Event>
where
    Given: Clone + Eq + Hash,
    Event: Clone + Eq + Hash,
{
    join(conditional, prior).marginalize(|(_, event)| event.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::Discrete;

    #[test]
    fn distribution_for_event_a1() {
        let expected = Discrete::from([("b1", 0.7), ("b2", 0.3)]);
        assert_eq!(distribution_b_given_a(&"a1"), expected);
    }

    #[test]
    fn distribution_for_event_a2() {
        let expected = Discrete::from([("b1", 0.2), ("b2", 0.8)]);
        assert_eq!(distribution_b_given_a(&"a2"), expected);
    }

    #[test]
    fn join_distributions() {
        assert_eq!(
            distribution_a_and_b(),
            Discrete::from([
                (("a1", "b1"), 0.63),
                (("a1", "b2"), 0.27),
                (("a2", "b1"), 0.02),
                (("a2", "b2"), 0.08),
            ])
        )
    }

    #[test]
    fn bayesian_evidence_for_positive_test() {
        assert_eq!(
            bayesian_evidence(
                &distribution_test_given_disease,
                &distribution_disease(),
                &Test::Positive
            ),
            Discrete::from(
                [(Disease::Sick, 0.497), (Disease::Healthy, 0.502),]
            )
        )
    }

    #[test]
    fn total_probability_for_test() {
        assert_eq!(
            total_probability(
                &distribution_test_given_disease,
                &distribution_disease()
            ),
            Discrete::from([(Test::Positive, 0.002), (Test::Negative, 0.998),])
        )
    }

    type A = &'static str;
    type B = &'static str;

    fn distribution_a() -> Discrete<A> {
        Discrete::from([("a1", 0.9), ("a2", 0.1)])
    }

    fn distribution_b_given_a(given: &A) -> Discrete<B> {
        if *given == "a1" {
            Discrete::from([("b1", 0.7), ("b2", 0.3)])
        } else {
            Discrete::from([("b1", 0.2), ("b2", 0.8)])
        }
    }

    fn distribution_a_and_b() -> Discrete<(A, B)> {
        join(&distribution_b_given_a, &distribution_a())
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum Disease {
        Sick,
        Healthy,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum Test {
        Positive,
        Negative,
    }

    fn distribution_disease() -> Discrete<Disease> {
        Discrete::from([(Disease::Sick, 0.001), (Disease::Healthy, 0.999)])
    }

    fn distribution_test_given_disease(given: &Disease) -> Discrete<Test> {
        match *given {
            Disease::Sick => Discrete::from([
                (Test::Positive, 0.990),
                (Test::Negative, 0.010),
            ]),
            Disease::Healthy => Discrete::from([
                (Test::Positive, 0.001),
                (Test::Negative, 0.999),
            ]),
        }
    }
}
