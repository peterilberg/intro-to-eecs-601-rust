use crate::distributions::Discrete;
use core::hash::Hash;

pub trait Conditional {
    type Given: Clone + Eq + Hash;
    type Event: Clone + Eq + Hash;

    fn when(&self, given: &Self::Given) -> &Discrete<Self::Event>;

    fn join(
        &self,
        distribution: &Discrete<Self::Given>,
    ) -> Discrete<(Self::Given, Self::Event)> {
        let joint_events = distribution.support().flat_map(|given| {
            let condition = distribution.probability(given);
            let conditioned = self.when(given);
            conditioned.support().map(move |event| {
                (
                    (given.clone(), event.clone()),
                    (condition * conditioned.probability(event)),
                )
            })
        });
        Discrete::from_iter(joint_events)
    }

    fn bayesian_evidence(
        &self,
        prior: &Discrete<Self::Given>,
        evidence: &Self::Event,
    ) -> Discrete<Self::Given> {
        self.join(prior)
            .condition(|(_, event)| event == evidence)
            .marginalize(|(event, _)| event.clone())
    }

    fn total_probability(
        &self,
        prior: &Discrete<Self::Given>,
    ) -> Discrete<Self::Event> {
        self.join(prior).marginalize(|(_, event)| event.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::Discrete;

    #[test]
    fn distribution_for_event_a1() {
        let expected = Discrete::from([("b1", 0.7), ("b2", 0.3)]);
        assert_eq!(distribution_b_given_a().when(&"a1"), &expected);
    }

    #[test]
    fn distribution_for_event_a2() {
        let expected = Discrete::from([("b1", 0.2), ("b2", 0.8)]);
        assert_eq!(distribution_b_given_a().when(&"a2"), &expected);
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
            distribution_test_given_disease()
                .bayesian_evidence(&distribution_disease(), &Test::Positive),
            Discrete::from(
                [(Disease::Sick, 0.497), (Disease::Healthy, 0.502),]
            )
        )
    }

    #[test]
    fn total_probability() {
        assert_eq!(
            distribution_test_given_disease()
                .total_probability(&distribution_disease()),
            Discrete::from([(Test::Positive, 0.002), (Test::Negative, 0.998),])
        )
    }

    type A = &'static str;
    type B = &'static str;

    fn distribution_a() -> Discrete<A> {
        Discrete::from([("a1", 0.9), ("a2", 0.1)])
    }

    fn distribution_b_given_a() -> BGivenA {
        BGivenA {
            distribution_a1: Discrete::from([("b1", 0.7), ("b2", 0.3)]),
            distribution_a2: Discrete::from([("b1", 0.2), ("b2", 0.8)]),
        }
    }

    fn distribution_a_and_b() -> Discrete<(A, B)> {
        distribution_b_given_a().join(&distribution_a())
    }

    struct BGivenA {
        distribution_a1: Discrete<B>,
        distribution_a2: Discrete<B>,
    }

    impl Conditional for BGivenA {
        type Given = A;
        type Event = B;

        fn when(&self, given: &Self::Given) -> &Discrete<Self::Event> {
            if *given == "a1" {
                &self.distribution_a1
            } else {
                &self.distribution_a2
            }
        }
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

    fn distribution_test_given_disease() -> TestGivenDisease {
        TestGivenDisease {
            sick: Discrete::from([
                (Test::Positive, 0.990),
                (Test::Negative, 0.010),
            ]),
            healthy: Discrete::from([
                (Test::Positive, 0.001),
                (Test::Negative, 0.999),
            ]),
        }
    }

    struct TestGivenDisease {
        sick: Discrete<Test>,
        healthy: Discrete<Test>,
    }

    impl Conditional for TestGivenDisease {
        type Given = Disease;
        type Event = Test;

        fn when(&self, given: &Self::Given) -> &Discrete<Self::Event> {
            match *given {
                Disease::Sick => &self.sick,
                Disease::Healthy => &self.healthy,
            }
        }
    }
}
