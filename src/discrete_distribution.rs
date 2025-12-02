use super::conditional_distribution::ConditionalDistribution;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    distribution: HashMap<Event, f64>,
}

impl<Event> DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from_events(events: HashMap<Event, f64>) -> DiscreteDistribution<Event> {
        assert_ne!(
            0,
            events.len(),
            "Discrete distribution must have at least one event."
        );

        let total: f64 = events.values().sum();
        assert!(
            0.0 < total,
            "Sum of event probabilities must be greater than 0."
        );

        let mut events = events;
        for probability in events.values_mut() {
            *probability /= total;
        }
        DiscreteDistribution {
            distribution: events,
        }
    }

    pub fn probability(&self, event: &Event) -> f64 {
        self.distribution.get(event).copied().unwrap_or(0.0)
    }

    pub fn support(&self) -> impl Iterator<Item = &Event> {
        self.distribution.keys()
    }

    pub fn draw(&self) -> &Event {
        let number: f64 = rand::rng().random_range(0.0..=1.0);
        let mut sum = 0.0;

        for event in self.support() {
            sum += self.probability(event);
            if number < sum {
                return event;
            }
        }
        self.support().last().unwrap()
    }

    pub fn join<AnotherEvent>(
        self,
        conditional_distribution: ConditionalDistribution<Event, AnotherEvent>,
    ) -> DiscreteDistribution<(Event, AnotherEvent)>
    where
        AnotherEvent: Copy + Eq + Hash,
    {
        let joined_events = self
            .support()
            .flat_map(|condition| {
                let probability = self.probability(condition);
                let distribution = conditional_distribution.when(condition)?;
                Some(((condition, probability), distribution))
            })
            .flat_map(|((condition, probability), distribution)| {
                distribution.support().map(move |event| {
                    (
                        (*condition, *event),
                        (probability * distribution.probability(event)),
                    )
                })
            });
        DiscreteDistribution::from_iter(joined_events)
    }

    pub fn marginalize<NewEvent>(
        self,
        convert_event: impl Fn(Event) -> NewEvent,
    ) -> DiscreteDistribution<NewEvent>
    where
        NewEvent: Copy + Eq + Hash,
    {
        let mut distribution = HashMap::new();
        for (old_event, probability) in self.distribution {
            let new_event = convert_event(old_event);
            let entry = distribution.entry(new_event).or_insert(0.0);
            *entry += probability;
        }

        DiscreteDistribution::from_events(distribution)
    }

    pub fn condition(self, condition: impl FnMut(&&Event) -> bool) -> Self {
        DiscreteDistribution::from_iter(
            self.support()
                .filter(condition)
                .map(|event| (*event, self.probability(event))),
        )
    }

    pub fn bayes<AnotherEvent>(
        self,
        conditional_distribution: ConditionalDistribution<Event, AnotherEvent>,
        evidence: &AnotherEvent,
    ) -> DiscreteDistribution<Event>
    where
        AnotherEvent: Copy + Eq + Hash,
    {
        self.join(conditional_distribution)
            .condition(|(_, another_event)| another_event == evidence)
            .marginalize(|(event, _)| event)
    }

    pub fn total_probability<AnotherEvent>(
        self,
        conditional_distribution: ConditionalDistribution<Event, AnotherEvent>,
    ) -> DiscreteDistribution<AnotherEvent>
    where
        AnotherEvent: Copy + Eq + Hash,
    {
        self.join(conditional_distribution)
            .marginalize(|(_, another_event)| another_event)
    }
}

impl<Event, const N: usize> From<[(Event, f64); N]>
    for DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from(events: [(Event, f64); N]) -> Self {
        let events = HashMap::from(events);
        DiscreteDistribution::from_events(events)
    }
}

impl<Event> From<Vec<(Event, f64)>> for DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from(events: Vec<(Event, f64)>) -> Self {
        let events = HashMap::from_iter(events);
        DiscreteDistribution::from_events(events)
    }
}

impl<Event> FromIterator<(Event, f64)> for DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from_iter<Iterator>(events: Iterator) -> Self
    where
        Iterator: IntoIterator<Item = (Event, f64)>,
    {
        let events = HashMap::from_iter(events);
        DiscreteDistribution::from_events(events)
    }
}

impl<Event> PartialEq for DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        let mut distribution1: Vec<_> = self.distribution.iter().collect();
        let mut distribution2: Vec<_> = other.distribution.iter().collect();
        distribution1.sort_by_key(|(event, _)| *event);
        distribution2.sort_by_key(|(event, _)| *event);
        distribution1.iter().zip(distribution2.iter()).all(
            |((event1, probability1), (event2, probability2))| {
                **event1 == **event2
                    && (**probability1 - **probability2).abs() < 0.001
            },
        )
    }
}

impl<Event> Eq for DiscreteDistribution<Event> where
    Event: Copy + Eq + Hash + Ord
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_of_distribution_a() {
        let d_a = distribution_a();
        let mut support: Vec<_> = d_a.support().collect();
        support.sort();
        assert_eq!(support, [&"a1", &"a2"]);
    }

    #[test]
    fn probability_of_events_in_distribution_a() {
        let d_a = distribution_a();
        assert_eq!(d_a.probability(&"a1"), 0.9);
        assert_eq!(d_a.probability(&"a2"), 0.1);
    }

    #[test]
    fn draw_from_distribution_a() {
        let d_a = distribution_a();
        let mut results = HashMap::new();
        for _ in 1..=100 {
            let event = *d_a.draw();
            *results.entry(event).or_insert(0) += 1;
        }
        assert!(results["a1"] > results["a2"]);
    }

    #[test]
    fn join_distributions() {
        assert_eq!(
            distribution_a_and_b(),
            DiscreteDistribution::from([
                (("a1", "b1"), 0.63),
                (("a1", "b2"), 0.27),
                (("a2", "b1"), 0.02),
                (("a2", "b2"), 0.08),
            ])
        )
    }

    #[test]
    fn marginalize_distribution_without_a() {
        assert_eq!(
            distribution_a_and_b().marginalize(|(_, b)| b),
            DiscreteDistribution::from([("b1", 0.65), ("b2", 0.35),])
        )
    }

    #[test]
    fn marginalize_distribution_without_b() {
        assert_eq!(
            distribution_a_and_b().marginalize(|(a, _)| a),
            DiscreteDistribution::from([("a1", 0.90), ("a2", 0.10),])
        )
    }

    #[test]
    fn conditional_distribution_to_b1() {
        assert_eq!(
            distribution_a_and_b()
                .condition(|(_, b)| b == &"b1")
                .marginalize(|(a, _)| a),
            DiscreteDistribution::from([("a1", 0.97), ("a2", 0.03),])
        )
    }

    #[test]
    fn bayesian_evidence_for_positive_test() {
        assert_eq!(
            distribution_disease()
                .bayes(distribution_test_given_disease(), &true),
            DiscreteDistribution::from([(true, 0.497), (false, 0.502),])
        )
    }

    #[test]
    fn total_probability() {
        assert_eq!(
            distribution_disease()
                .total_probability(distribution_test_given_disease()),
            DiscreteDistribution::from([(true, 0.002), (false, 0.998),])
        )
    }

    fn distribution_a() -> DiscreteDistribution<&'static str> {
        DiscreteDistribution::from([("a1", 0.9), ("a2", 0.1)])
    }

    fn distribution_b_given_a()
    -> ConditionalDistribution<&'static str, &'static str> {
        ConditionalDistribution::from([
            ("a1", DiscreteDistribution::from([("b1", 0.7), ("b2", 0.3)])),
            ("a2", DiscreteDistribution::from([("b1", 0.2), ("b2", 0.8)])),
        ])
    }

    fn distribution_a_and_b()
    -> DiscreteDistribution<(&'static str, &'static str)> {
        distribution_a().join(distribution_b_given_a())
    }

    fn distribution_disease() -> DiscreteDistribution<bool> {
        DiscreteDistribution::from([(true, 0.001), (false, 0.999)])
    }

    fn distribution_test_given_disease() -> ConditionalDistribution<bool, bool>
    {
        ConditionalDistribution::from([
            (
                true,
                DiscreteDistribution::from([(true, 0.990), (false, 0.010)]),
            ),
            (
                false,
                DiscreteDistribution::from([(true, 0.001), (false, 0.999)]),
            ),
        ])
    }
}
